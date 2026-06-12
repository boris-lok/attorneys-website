use crate::domain::users::entity::UserID;

#[async_trait::async_trait]
pub trait SessionStore {
    async fn clear_user_sessions(&self, user_id: &UserID) -> anyhow::Result<()>;
    async fn create_session(&self, key: &str, value: &str, expired_sec: u64) -> anyhow::Result<()>;
    async fn get_session(&self, key: &str) -> anyhow::Result<Option<String>>;
}
