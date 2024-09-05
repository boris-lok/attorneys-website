use crate::domain::entities::{ContentData, ContentID, Language};
use sqlx::{Acquire, Postgres, Transaction};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum InsertError {
    Conflict,
    Unknown,
}
#[async_trait::async_trait]
pub trait IContentRepository {
    async fn insert(
        &self,
        content_id: ContentID,
        content: ContentData,
        language: Language,
    ) -> Result<ContentID, InsertError>;
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
}

#[async_trait::async_trait]
impl IContentRepository for InMemoryContentRepository {
    async fn insert(
        &self,
        content_id: ContentID,
        content: ContentData,
        language: Language,
    ) -> Result<ContentID, InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut lock = self.content.lock().await;

        let key = format!("{}_{:?}", content_id.clone().0, language);
        if lock.contains_key(&key) {
            return Err(InsertError::Conflict);
        }

        lock.insert(key, content);

        Ok(content_id)
    }
}

#[derive(Debug)]
pub struct SqlxContentRepository<'tx> {
    tx: Arc<Mutex<Transaction<'tx, Postgres>>>,
}

impl<'tx> SqlxContentRepository<'tx> {
    pub fn new(tx: Arc<Mutex<Transaction<'tx, Postgres>>>) -> Self {
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
    ) -> Result<ContentID, InsertError> {
        let mut lock = self.tx.lock().await;
        let conn = lock.acquire().await.unwrap();

        sqlx::query(
            "INSERT INTO \"content\" (id, data, language, created_at) VALUES ($1, $2, $3, now());",
        )
        .bind(content_id.0.clone())
        .bind(content.0)
        .bind(language.as_str())
        .execute(conn)
        .await
        .map_err(|e| {
            println!("Failed to insert content: {:?}", e);

            InsertError::Unknown
        })?;

        Ok(content_id)
    }
}
