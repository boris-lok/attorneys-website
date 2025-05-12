use crate::repositories::Connection;
use anyhow::anyhow;
use sqlx::{Acquire, PgConnection};
use std::net::IpAddr;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IArticleViewsRepository {
    async fn save(
        &self,
        article_id: String,
        ip: IpAddr,
        user_agent: String,
    ) -> anyhow::Result<Uuid>;
}

pub struct InMemoryArticleViewsRepository {
    error: bool,
    data: Mutex<Vec<(Uuid, String, IpAddr, String)>>,
}

#[cfg(test)]
impl Default for InMemoryArticleViewsRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryArticleViewsRepository {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            error: false,
            data: Mutex::new(Vec::new()),
        }
    }

    #[cfg(test)]
    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

#[async_trait::async_trait]
impl IArticleViewsRepository for InMemoryArticleViewsRepository {
    async fn save(
        &self,
        article_id: String,
        ip: IpAddr,
        user_agent: String,
    ) -> anyhow::Result<Uuid> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut data = self.data.lock().await;
        let uuid = Uuid::new_v4();
        data.push((uuid, article_id, ip, user_agent));

        Ok(uuid)
    }
}

#[derive(Debug)]
pub struct SqlxArticleViewsRepository<'tx> {
    conn: Connection<'tx>,
}

impl<'tx> SqlxArticleViewsRepository<'tx> {
    pub fn new(conn: Connection<'tx>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl IArticleViewsRepository for SqlxArticleViewsRepository<'_> {
    async fn save(
        &self,
        article_id: String,
        ip: IpAddr,
        user_agent: String,
    ) -> anyhow::Result<Uuid> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();
                let id = save(conn, article_id, ip, user_agent).await?;
                Ok(id)
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                let id = save(conn, article_id, ip, user_agent).await?;
                Ok(id)
            }
        }
    }
}

async fn save(
    c: &mut PgConnection,
    article_id: String,
    ip: IpAddr,
    user_agent: String,
) -> anyhow::Result<Uuid> {
    let id = sqlx::query_scalar::<_, Uuid>(
        "insert into \"article_views\" (article_id, viewer_ip, user_agent) values ($1, $2, $3) returning id;",
    )
        .bind(article_id.as_str())
        .bind(ip)
        .bind(user_agent)
        .fetch_one(c)
        .await?;

    Ok(id)
}
