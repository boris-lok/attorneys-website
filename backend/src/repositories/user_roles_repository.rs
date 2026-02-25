use crate::domain::entities::UserID;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IUserRolesRepository {
    async fn insert_user_role(&mut self, user_id: UserID, role_id: i16) -> anyhow::Result<()>;
}

pub struct SqlxUserRolesRepository<'tx> {
    conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>,
}

impl<'tx> SqlxUserRolesRepository<'tx> {
    pub fn new(conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl IUserRolesRepository for SqlxUserRolesRepository<'_> {
    async fn insert_user_role(&mut self, user_id: UserID, role_id: i16) -> anyhow::Result<()> {
        let id = Uuid::parse_str(user_id.to_string().as_str())?;

        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = "insert into user_roles (user_id, role_id) values ($1, $2)";

        sqlx::query(query)
            .bind(id)
            .bind(role_id)
            .execute(conn)
            .await?;

        Ok(())
    }
}
