use anyhow::anyhow;
use sqlx::{Acquire, Postgres, Transaction};
use std::net::IpAddr;
use std::sync::Weak;
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
    tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
}

impl<'tx> SqlxArticleViewsRepository<'tx> {
    pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
        Self { tx }
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
        let conn_ptr = self
            .tx
            .upgrade()
            .ok_or_else(|| anyhow!("can't upgrade the transaction"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        let id = sqlx::query_scalar::<_, Uuid>(
            "insert into \"article_views\" (article_id, view_ip, user_agent) values ($1, $2, $3) returning id;",
        )
        .bind(article_id.as_str())
        .bind(ip.to_string())
        .bind(user_agent)
        .fetch_one(conn)
        .await?;

        Ok(id)
    }
}
