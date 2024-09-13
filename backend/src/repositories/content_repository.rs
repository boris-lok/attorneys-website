use crate::domain::entities::{ContentData, ContentID, Language};
use anyhow::anyhow;
use sqlx::{Acquire, Postgres, Transaction};
use std::collections::HashMap;
use std::sync::Weak;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IContentRepository {
    async fn insert(
        &self,
        content_id: ContentID,
        content: ContentData,
        language: Language,
    ) -> anyhow::Result<ContentID>;

    async fn update(
        &self,
        content_id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct InMemoryContentRepository {
    error: bool,
    content: Mutex<HashMap<String, ContentData>>,
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
}

#[async_trait::async_trait]
impl IContentRepository for InMemoryContentRepository {
    async fn insert(
        &self,
        content_id: ContentID,
        content: ContentData,
        language: Language,
    ) -> anyhow::Result<ContentID> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.content.lock().await;

        let key = format!("{}_{}", content_id.as_str(), language.as_str());
        if lock.contains_key(&key) {
            return Err(anyhow!("{} already exists", content_id.as_str()));
        }

        lock.insert(key, content);

        Ok(content_id)
    }

    async fn update(
        &self,
        content_id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.content.lock().await;

        let key = format!("{}_{}", content_id.as_str(), language.as_str());
        if !lock.contains_key(&key) {
            return Err(anyhow!("{} doesn't exists", content_id.as_str()));
        }

        lock.entry(key).and_modify(|e| *e = data);

        Ok(())
    }
}

#[derive(Debug)]
pub struct SqlxContentRepository<'tx> {
    tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
}

impl<'tx> SqlxContentRepository<'tx> {
    pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl<'tx> IContentRepository for SqlxContentRepository<'tx> {
    async fn insert(
        &self,
        content_id: ContentID,
        content: ContentData,
        language: Language,
    ) -> anyhow::Result<ContentID> {
        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        sqlx::query(
            "INSERT INTO \"content\" (id, data, language, created_at) VALUES ($1, $2, $3, now());",
        )
        .bind(content_id.as_str())
        .bind(content.0)
        .bind(language.as_str())
        .execute(conn)
        .await?;

        Ok(content_id)
    }

    async fn update(
        &self,
        content_id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
