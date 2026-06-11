use crate::domain::session::store::SessionStore;
use crate::domain::users::entity::UserID;
use redis::Commands;

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

        c.del::<_, ()>(user_id.to_string())?;

        Ok(())
    }

    async fn create_session(&self, user_id: &UserID, exp: i64) -> anyhow::Result<()> {
        let mut c = self.redis.get_connection()?;

        c.set::<_, _, ()>(user_id.to_string(), exp)?;

        Ok(())
    }
}
