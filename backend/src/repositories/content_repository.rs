use crate::domain::entities::{ContentData, ContentID, Language};
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;
use std::sync::Mutex;

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

        let mut lock = match self.content.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown),
        };

        let key = format!("{}_{:?}", content_id.clone().0, language);
        if lock.contains_key(&key) {
            return Err(InsertError::Conflict);
        }

        lock.insert(key, content);

        Ok(content_id)
    }
}

pub struct SqlxContentRepository {}

impl SqlxContentRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl IContentRepository for SqlxContentRepository {
    async fn insert(
        &self,
        content_id: ContentID,
        content: ContentData,
        language: Language,
    ) -> Result<ContentID, InsertError> {
        todo!()
    }
}
