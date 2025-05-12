use crate::domain::entities::{ContentData, ContentID, Language};
use crate::repositories::Connection;
use anyhow::anyhow;
use sqlx::{Acquire, PgConnection, Row};
use std::collections::HashMap;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IContentRepository {
    async fn insert(
        &self,
        id: ContentID,
        content: ContentData,
        language: Language,
    ) -> anyhow::Result<ContentID>;

    async fn update(
        &self,
        id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()>;

    async fn contains(&self, id: &ContentID, language: &Language) -> anyhow::Result<bool>;
}

#[derive(Debug)]
pub struct InMemoryContentRepository {
    error: bool,
    content: Mutex<HashMap<String, ContentData>>,
}

impl Default for InMemoryContentRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryContentRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            content: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }

    pub async fn get(
        &self,
        id: &ContentID,
        language: &Language,
    ) -> anyhow::Result<Option<ContentData>> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let lock = self.content.lock().await;
        let key = format!("{}_{}", id.as_str(), language.as_str());

        Ok(lock.get(&key).cloned())
    }

    pub async fn list(&self, language: &Language) -> anyhow::Result<Vec<(String, ContentData)>> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let lock = self.content.lock().await;
        let values = lock
            .iter()
            .filter(|(key, _)| key.ends_with(language.as_str()))
            .map(|(key, value)| {
                let id = key.split("_").next().unwrap().to_string();
                (id, value.clone())
            })
            .collect::<Vec<_>>();

        Ok(values)
    }
}

#[async_trait::async_trait]
impl IContentRepository for InMemoryContentRepository {
    async fn insert(
        &self,
        id: ContentID,
        content: ContentData,
        language: Language,
    ) -> anyhow::Result<ContentID> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.content.lock().await;

        let key = format!("{}_{}", id.as_str(), language.as_str());
        if lock.contains_key(&key) {
            return Err(anyhow!("{} already exists", id.as_str()));
        }

        lock.insert(key, content);

        Ok(id)
    }

    async fn update(
        &self,
        id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.content.lock().await;

        let key = format!("{}_{}", id.as_str(), language.as_str());
        if !lock.contains_key(&key) {
            return Err(anyhow!("{} doesn't exists", id.as_str()));
        }

        lock.entry(key).and_modify(|e| *e = data);

        Ok(())
    }

    async fn contains(&self, id: &ContentID, language: &Language) -> anyhow::Result<bool> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let lock = self.content.lock().await;

        let key = format!("{}_{}", id.as_str(), language.as_str());
        Ok(lock.contains_key(key.as_str()))
    }
}

#[derive(Debug)]
pub struct SqlxContentRepository<'tx> {
    conn: Connection<'tx>,
}

impl<'tx> SqlxContentRepository<'tx> {
    pub fn new(conn: Connection<'tx>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl IContentRepository for SqlxContentRepository<'_> {
    async fn insert(
        &self,
        id: ContentID,
        content: ContentData,
        language: Language,
    ) -> anyhow::Result<ContentID> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                create(conn, id, content, language).await
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                create(conn, id, content, language).await
            }
        }
    }

    async fn update(
        &self,
        id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                update(conn, id, data, language).await
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                update(conn, id, data, language).await
            }
        }
    }

    async fn contains(&self, id: &ContentID, language: &Language) -> anyhow::Result<bool> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                contains(conn, id, language).await
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                contains(conn, id, language).await
            }
        }
    }
}

async fn create(
    conn: &mut PgConnection,
    id: ContentID,
    content: ContentData,
    language: Language,
) -> anyhow::Result<ContentID> {
    sqlx::query(
        "INSERT INTO \"content\" (id, data, language, created_at, updated_at) VALUES ($1, $2, $3, now(), now());",
    )
        .bind(id.as_str())
        .bind(content.as_json())
        .bind(language.as_str())
        .execute(conn)
        .await?;

    Ok(id)
}

async fn update(
    conn: &mut PgConnection,
    id: &ContentID,
    data: ContentData,
    language: Language,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE \"content\" SET data = $1, updated_at = now() WHERE id = $2 AND language = $3;",
    )
    .bind(data.as_json())
    .bind(id.as_str())
    .bind(language.as_str())
    .execute(conn)
    .await?;

    Ok(())
}

async fn contains(
    conn: &mut PgConnection,
    id: &ContentID,
    language: &Language,
) -> anyhow::Result<bool> {
    let res = sqlx::query("SELECT id FROM \"content\" WHERE id = $1 AND language = $2 limit 1;")
        .bind(id.as_str())
        .bind(language.as_str())
        .fetch_optional(conn)
        .await
        .map(|row| match row {
            None => false,
            Some(row) => row.len() > 0,
        })?;

    Ok(res)
}
