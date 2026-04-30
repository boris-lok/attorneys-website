use std::collections::HashMap;
use crate::domain::entities::UserID;
use crate::repositories::CaseID;
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
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
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

impl From<WorkLogStatus> for String {
    fn from(value: WorkLogStatus) -> String {
        match value {
            WorkLogStatus::Pending => "pending".to_string(),
            WorkLogStatus::Rejected => "rejected".to_string(),
            WorkLogStatus::Approved => "approved".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WorkLog {
    pub id: Uuid,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub user: SimpleUser,
    pub duration: i32,
    pub description: String,
    pub is_collaborative: bool,
    pub collaborators: Vec<Collaborator>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct SimpleUser {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Collaborator {
    pub parent_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub status: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct WorkLogFromSQLx {
    id: Uuid,
    started_at: chrono::DateTime<chrono::Utc>,
    user_id: Uuid,
    username: String,
    duration: i32,
    description: String,
    is_collaborative: bool,
    status: WorkLogStatus,
    parent_id: Option<Uuid>,
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
          wl.case_id,
          wl.started_at,
          wl.user_id,
          u.nickname as username,
          wl.duration_minutes as duration,
          wl.description,
          wl.is_collaborative,
          wl.status,
          wl.parent_id
        from work_logs wl
        join users u on u.id = wl.user_id
        left join work_logs child on child.parent_id = wl.id
        where
          wl.case_id = $1
          and wl.deleted_at is null
        order by
          wl.started_at;
        ";

        let rows = sqlx::query_as::<_, WorkLogFromSQLx>(query)
            .bind(Uuid::from(case_id.clone()))
            .fetch_all(conn)
            .await?;

        let (parents, childs): (Vec<_>, Vec<_>) =
            rows.into_iter().partition(|row| row.parent_id.is_none());

        // 1. Group children by parent_id
        let mut grouped: HashMap<_, Vec<_>> = HashMap::new();
        for child in childs {
            if let Some(pid) = child.parent_id {
                grouped.entry(pid).or_default().push(child);
            }
        }

        // 2. Build result
        let res: Vec<_> = parents.into_iter().map(|parent| {
            let collaborators = grouped
                .remove(&parent.id)
                .unwrap_or_default()
                .into_iter()
                .map(|child| Collaborator {
                    parent_id: parent.id,
                    user_id: child.user_id,
                    name: child.username,          // no clone
                    status: child.status.into(),   // no clone
                })
                .collect();

            WorkLog {
                id: parent.id,
                started_at: parent.started_at,
                user: SimpleUser {
                    id: parent.user_id,
                    name: parent.username,
                },
                duration: parent.duration,
                description: parent.description,
                is_collaborative: parent.is_collaborative,
                collaborators,
                status: parent.status.into(),
            }
        }).collect();

        Ok(res)
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
        deleted_at = coalesce($5, deleted_at)
        where id = $6;
        ";

        sqlx::query(query)
            .bind(req.status)
            .bind(req.started_at)
            .bind(req.ended_at)
            .bind(req.description)
            .bind(req.deleted_at)
            .bind(req.id)
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn is_creator(&self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;
        let user_id = Uuid::from(user_id);

        let query = r"select id from work_logs where user_id = $1 and id = $2";

        let row = sqlx::query(query)
            .bind(user_id)
            .bind(id)
            .fetch_one(conn)
            .await?;

        Ok(!row.is_empty())
    }

    async fn is_collaborator_work_log(&self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;
        let user_id = Uuid::from(user_id);

        let query = r"select is_collaborative from work_logs where id = $1 and user_id = $2";

        let is_collaborator: bool = sqlx::query_scalar(query)
            .bind(id)
            .bind(user_id)
            .fetch_one(conn)
            .await?;

        Ok(is_collaborator)
    }

    async fn is_work_log_exist(&self, id: &Uuid) -> anyhow::Result<bool> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"select id from work_logs where id = $1";

        let row = sqlx::query(query).bind(id).fetch_one(conn).await?;

        Ok(!row.is_empty())
    }
}
