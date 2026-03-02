use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct CaseID(Uuid);

impl From<CaseID> for String {
    fn from(id: CaseID) -> String {
        id.0.to_string()
    }
}

#[async_trait::async_trait]
pub trait ICaseRepository {
    async fn create_case(&mut self, name: &str, estimated_minutes: i32) -> anyhow::Result<CaseID>;
}

pub struct SQLxCaseRepository<'tx> {
    conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>,
}

impl<'tx> SQLxCaseRepository<'tx> {
    pub fn new(conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl ICaseRepository for SQLxCaseRepository<'_> {
    async fn create_case(&mut self, name: &str, estimated_minutes: i32) -> anyhow::Result<CaseID> {
        let id = Uuid::new_v4();

        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"insert into case (id, name, estimated_minutes) values ($1, $2, $3)";

        sqlx::query(query)
            .bind(id)
            .bind(name)
            .bind(estimated_minutes)
            .execute(conn)
            .await?;

        Ok(CaseID(id))
    }
}
