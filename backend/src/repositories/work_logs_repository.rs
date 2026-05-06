use crate::domain::entities::UserID;
use crate::repositories::CaseID;
use serde::Serialize;
use sqlx::{Postgres, Row};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IWorkLogsRepository {
    async fn create(&mut self, work_log: CreateWorkLog) -> anyhow::Result<()>;
    async fn create_mapping(&mut self, id: Uuid, user_ids: Vec<UserID>) -> anyhow::Result<()>;
    async fn delete(&mut self, id: Uuid) -> anyhow::Result<()>;
    async fn list(&self, case_id: &CaseID) -> anyhow::Result<Vec<WorkLog>>;
    async fn update(&mut self, req: UpdateWorkLog) -> anyhow::Result<()>;
    async fn update_status(
        &mut self,
        id: &Uuid,
        user_id: &UserID,
        status: WorkLogStatus,
    ) -> anyhow::Result<()>;
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
    pub ended_at: chrono::DateTime<chrono::Utc>,
    pub user: SimpleUser,
    pub duration: i32,
    pub description: String,
    pub is_collaborative: bool,
    pub collaborators: Vec<Collaborator>,
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
    ended_at: chrono::DateTime<chrono::Utc>,
    user_id: Uuid,
    username: String,
    duration: i32,
    description: String,
    is_collaborative: bool,
    status: Option<WorkLogStatus>,
    parent_id: Option<Uuid>,
    collaborator_user_id: Option<Uuid>,
    collaborator_name: Option<String>,
}

#[async_trait::async_trait]
impl IWorkLogsRepository for SqlxWorkLogsRepository<'_> {
    async fn create(&mut self, work_log: CreateWorkLog) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        insert into work_logs (id, user_id, case_id, started_at, ended_at, description, is_collaborative)
        values ($1, $2, $3, $4, $5, $6, $7)
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
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn create_mapping(&mut self, id: Uuid, user_ids: Vec<UserID>) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let mut qb = sqlx::QueryBuilder::<Postgres>::new(
            "insert into work_logs_mapping (parent_id, user_id, status)",
        );

        qb.push_values(user_ids, |mut b, user_id| {
            let user_id = Uuid::from(&user_id);
            b.push_bind(id)
                .push_bind(user_id)
                .push_bind(WorkLogStatus::Pending);
        });

        let query = qb.build();
        query.execute(conn).await?;

        Ok(())
    }

    async fn delete(&mut self, id: Uuid) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        update work_logs set deleted_at = now() where id = $1;
        ";

        sqlx::query(query).bind(id).execute(conn).await?;

        Ok(())
    }

    async fn list(&self, case_id: &CaseID) -> anyhow::Result<Vec<WorkLog>> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        select
          wl.id,
          wl.case_id,
          wl.started_at,
          wl.ended_at,
          wl.user_id,
          u.nickname as username,
          wl.duration_minutes as duration,
          wl.description,
          wl.is_collaborative,
          wlm.status,
          wlm.parent_id,
          wlm.user_id as collaborator_user_id,
          cu.nickname as collaborator_name
        from work_logs wl
        join users u on u.id = wl.user_id
        left join work_logs_mapping wlm on wlm.parent_id = wl.id
        left join users cu on wlm.user_id = cu.id
        where
          wl.case_id = $1
          and wl.deleted_at is null
        ";

        let rows = sqlx::query_as::<_, WorkLogFromSQLx>(query)
            .bind(Uuid::from(case_id.clone()))
            .fetch_all(conn)
            .await?;

        let mut res: HashMap<_, _> = HashMap::new();
        for row in rows {
            // Ensure parent WorkLog exists
            res.entry(row.id).or_insert_with(|| WorkLog {
                id: row.id,
                started_at: row.started_at,
                ended_at: row.ended_at,
                user: SimpleUser {
                    id: row.user_id,
                    name: row.username,
                },
                duration: row.duration,
                description: row.description,
                is_collaborative: row.is_collaborative,
                collaborators: vec![],
            });

            // Attach collaborator if present
            if let Some(parent_id) = row.parent_id {
                if let Some(parent) = res.get_mut(&parent_id) {
                    parent.collaborators.push(Collaborator {
                        parent_id,
                        user_id: row.collaborator_user_id.unwrap(),
                        name: row.collaborator_name.unwrap(),
                        status: row.status.unwrap().into(),
                    });
                }
            }
        }

        let mut sorted: Vec<_> = res.into_values().collect();
        sorted.sort_by_key(|wl| wl.started_at);

        Ok(sorted)
    }

    async fn update(&mut self, req: UpdateWorkLog) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        update work_logs set
        started_at = coalesce($1, started_at),
        ended_at = coalesce($2, ended_at),
        description = coalesce($3, description),
        deleted_at = coalesce($4, deleted_at)
        where id = $5;
        ";

        sqlx::query(query)
            .bind(req.started_at)
            .bind(req.ended_at)
            .bind(req.description)
            .bind(req.deleted_at)
            .bind(req.id)
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn update_status(
        &mut self,
        id: &Uuid,
        user_id: &UserID,
        status: WorkLogStatus,
    ) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;
        let user_id = Uuid::from(user_id);

        let query = r"
        update work_logs_mapping
        set status = $1
        where parent_id = $2 and user_id = $3;
        ";

        sqlx::query(query)
            .bind(status)
            .bind(id)
            .bind(user_id)
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
