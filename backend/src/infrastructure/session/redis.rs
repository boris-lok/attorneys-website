use crate::domain::session::store::SessionStore;
use crate::domain::users::entity::UserID;
use redis::TypedCommands;

pub struct RedisSessionStore {
    redis: redis::Client,
}

impl RedisSessionStore {
    pub fn new(redis: redis::Client) -> Self {
        Self { redis }
    }
}

#[async_trait::async_trait]
impl SessionStore for RedisSessionStore {
    async fn clear_user_sessions(&self, user_id: &UserID) -> anyhow::Result<()> {
        let mut c = self.redis.get_connection()?;

        c.del(user_id.to_string())?;

        Ok(())
    }

    async fn create_session(&self, key: &str, value: &str, expired_sec: u64) -> anyhow::Result<()> {
        let mut c = self.redis.get_connection()?;

        c.set_ex(key, value, expired_sec)?;

        Ok(())
    }

    async fn get_session(&self, key: &str) -> anyhow::Result<Option<String>> {
        let mut c = self.redis.get_connection()?;

        let res = c.get(key)?;

        Ok(res)
    }
}
