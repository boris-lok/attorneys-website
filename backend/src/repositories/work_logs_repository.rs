use crate::repositories::CaseID;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IWorkLogsRepository {
    async fn create_work_log(&mut self, work_log: WorkLog) -> anyhow::Result<()>;
}

pub struct SqlxWorkLogsRepository<'tx> {
    conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>,
}

impl<'tx> SqlxWorkLogsRepository<'tx> {
    pub fn new(conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>) -> Self {
        Self { conn }
    }
}

#[derive(Debug, Clone)]
pub struct WorkLog {
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

#[async_trait::async_trait]
impl IWorkLogsRepository for SqlxWorkLogsRepository<'_> {
    async fn create_work_log(&mut self, work_log: WorkLog) -> anyhow::Result<()> {
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
}
