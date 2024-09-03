use crate::domain::entities::{ContentData, ContentID, Language};
use std::collections::HashMap;
use std::sync::Mutex;

pub enum InsertError {
    Conflict,
    Unknown,
}
pub trait IContentRepository {
    fn insert(
        &self,
        content_id: ContentID,
        content: ContentData,
        language: Language,
    ) -> Result<ContentID, InsertError>;
}

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

impl IContentRepository for InMemoryContentRepository {
    fn insert(
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
