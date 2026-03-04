use crate::domain::entities::UserID;
use crate::repositories::{Case, CaseID};
use serde::Serialize;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IWorkLogsRepository {
    async fn create_work_log(&mut self, work_log: CreateWorkLog) -> anyhow::Result<()>;
    async fn list_work_logs(&self, case_id: &CaseID) -> anyhow::Result<Vec<WorkLog>>;
    async fn update_work_log(&mut self, req: UpdateWorkLog) -> anyhow::Result<()>;
    async fn is_creator(&self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool>;
    async fn is_collaborator_work_log(&self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool>;
    async fn is_work_log_exist(&self, id: &Uuid) -> anyhow::Result<bool>;
}

pub struct SqlxWorkLogsRepository<'tx> {
    conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>,
}

impl<'tx> SqlxWorkLogsRepository<'tx> {
    pub fn new(conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>) -> Self {
        Self { conn }
    }
}

pub struct UpdateWorkLog {
    pub id: Uuid,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub description: Option<String>,
    pub status: Option<WorkLogStatus>,
}

#[derive(Debug, Clone)]
pub struct CreateWorkLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub case_id: CaseID,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub is_collaborative: bool,
    pub parent_id: Option<Uuid>,
    pub status: WorkLogStatus,
}

#[derive(sqlx::Type, Clone, Debug)]
#[sqlx(type_name = "work_log_status", rename_all = "lowercase")]
pub enum WorkLogStatus {
    Pending,
    Rejected,
    Approved,
}

impl TryFrom<String> for WorkLogStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pending" => Ok(WorkLogStatus::Pending),
            "rejected" => Ok(WorkLogStatus::Rejected),
            "approved" => Ok(WorkLogStatus::Approved),
            _ => Err(format!("Invalid work log status: {}", value)),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WorkLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub case: Case,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub duration: i32,
    pub description: String,
    pub is_collaborative: bool,
    pub collaborators: Vec<Collaborator>,
}

#[derive(Debug, Serialize)]
pub struct Collaborator {
    pub user_id: Uuid,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct WorkLogFromSQLx {
    id: Uuid,
    user_id: Uuid,
    creator_name: String,
    case_id: Uuid,
    case_name: String,
    case_estimated_minutes: i32,
    case_created_at: chrono::DateTime<chrono::Utc>,
    started_at: chrono::DateTime<chrono::Utc>,
    duration: i32,
    description: String,
    collaborator_ids: Option<Vec<Uuid>>,
    collaborator_names: Option<Vec<String>>,
}

impl From<WorkLogFromSQLx> for WorkLog {
    fn from(value: WorkLogFromSQLx) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            case: Case {
                id: CaseID::from(value.case_id),
                name: value.case_name,
                estimated_minutes: value.case_estimated_minutes,
                created_at: value.case_created_at,
            },
            started_at: value.started_at,
            duration: value.duration,
            description: value.description,
            is_collaborative: value.collaborator_ids.is_some(),
            collaborators: value
                .collaborator_ids
                .map(|ids| {
                    ids.into_iter()
                        .zip(value.collaborator_names.unwrap())
                        .map(|(id, name)| Collaborator { user_id: id, name })
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

#[async_trait::async_trait]
impl IWorkLogsRepository for SqlxWorkLogsRepository<'_> {
    async fn create_work_log(&mut self, work_log: CreateWorkLog) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        insert into work_logs (id, user_id, case_id, started_at, ended_at, description, is_collaborative, parent_id, status)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        returning id;
        ";

        sqlx::query(query)
            .bind(work_log.id)
            .bind(work_log.user_id)
            .bind(Uuid::from(work_log.case_id))
            .bind(work_log.started_at)
            .bind(work_log.ended_at)
            .bind(work_log.description)
            .bind(work_log.is_collaborative)
            .bind(work_log.parent_id)
            .bind(work_log.status)
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn list_work_logs(&self, case_id: &CaseID) -> anyhow::Result<Vec<WorkLog>> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        select
          wl.id,
          wl.user_id,
          u.name as creator_name,
          wl.case_id,
          c.name as case_name,
          c.estimated_minutes as case_estimated_minutes,
          c.created_at as case_created_at,
          wl.started_at,
          extract(epoch from wl.ended_at - wl.started_at) as duration,
          wl.description,
          array_agg(distinct child.user_id) filter (where child.parent_id is not null and child.status = 'approved' and child.user_id is not null) as collaborator_ids,
          array_agg(distinct cu.name) filter (where child.parent_id is not null and child.status = 'approved' and child.user_id is not null) as collaborator_names
        from work_logs wl
        join cases c on c.id = wl.case_id
        join users u on u.id = wl.user_id
        left join work_logs child on child.parent_id = wl.id
        left join users cu on cu.id = child.user_id
        where
          c.id = $1
          and wl.deleted_at is null
          and wl.parent_id is null
        group by
          wl.id, wl.user_id, u.name, c.id
        order by
          wl.started_at;
        ";

        let rows = sqlx::query_as::<_, WorkLogFromSQLx>(query)
            .bind(Uuid::from(case_id.clone()))
            .fetch_all(conn)
            .await?;

        Ok(rows.into_iter().map(WorkLog::from).collect())
    }

    async fn update_work_log(&mut self, req: UpdateWorkLog) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        update work_logs
        set status = coalesce($1, status),
        started_at = coalesce($2, started_at),
        ended_at = coalesce($3, ended_at),
        description = coalesce($4, description)
        where id = $5;
        ";

        sqlx::query(query)
            .bind(req.status)
            .bind(req.started_at)
            .bind(req.ended_at)
            .bind(req.description)
            .bind(req.id)
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn is_creator(&self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"select wl.id from work_logs where user_id = $1 and id = $2";

        let row = sqlx::query(query)
            .bind(user_id.to_string())
            .bind(id)
            .fetch_one(conn)
            .await?;

        Ok(!row.is_empty())
    }

    async fn is_collaborator_work_log(&self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"select is_collaborator from work_logs where id = $1 and user_id = $2";

        let is_collaborator: bool = sqlx::query_scalar(query)
            .bind(id)
            .bind(user_id.to_string())
            .fetch_one(conn)
            .await?;

        Ok(is_collaborator)
    }

    async fn is_work_log_exist(&self, id: &Uuid) -> anyhow::Result<bool> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"select wl.id from work_logs where id = $1";

        let row = sqlx::query(query).bind(id).fetch_one(conn).await?;

        Ok(!row.is_empty())
    }
}
