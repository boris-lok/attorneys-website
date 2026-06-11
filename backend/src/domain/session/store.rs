use crate::domain::users::entity::UserID;

#[async_trait::async_trait]
pub trait SessionStore {
    async fn clear_user_sessions(&self, user_id: &UserID) -> anyhow::Result<()>;
    async fn create_session(&self, user_id: &UserID, exp: i64) -> anyhow::Result<()>;
}
