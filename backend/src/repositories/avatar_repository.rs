use crate::domain::entities::ResourceID;
use crate::domain::member::entities::AvatarJson;
use anyhow::anyhow;
use sqlx::{Acquire, Postgres, Transaction};
use std::collections::HashMap;
use std::sync::Weak;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum InsertError {
    Conflict,
    Unknown,
}
#[async_trait::async_trait]
pub trait IAvatarRepository {
    async fn save(
        &self,
        id: ResourceID,
        avatar_json: AvatarJson,
    ) -> Result<ResourceID, InsertError>;
}

#[derive(Debug)]
pub struct InMemoryAvatarRepository {
    error: bool,
    content: Mutex<HashMap<String, AvatarJson>>,
}

impl InMemoryAvatarRepository {
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

    pub async fn get(&self, id: &ResourceID) -> anyhow::Result<Option<AvatarJson>> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let lock = self.content.lock().await;

        Ok(lock.get(id.as_str()).cloned())
    }
}

#[async_trait::async_trait]
impl IAvatarRepository for InMemoryAvatarRepository {
    async fn save(
        &self,
        id: ResourceID,
        avatar_json: AvatarJson,
    ) -> Result<ResourceID, InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut lock = self.content.lock().await;

        if lock.contains_key(id.as_str()) {
            return Err(InsertError::Conflict);
        }

        {
            let id = id.to_string();
            lock.insert(id.clone(), avatar_json);
        }

        Ok(id)
    }
}

#[derive(Debug)]
pub struct SqlxAvatarRepository<'tx> {
    tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
}

impl<'tx> SqlxAvatarRepository<'tx> {
    pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl IAvatarRepository for SqlxAvatarRepository<'_> {
    async fn save(
        &self,
        id: ResourceID,
        avatar_json: AvatarJson,
    ) -> Result<ResourceID, InsertError> {
        let conn_ptr = self.tx.upgrade().ok_or(InsertError::Unknown)?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await.unwrap();

        sqlx::query("insert into \"avatar\" (id, data) values ($1, $2) on conflict (id) do update set data = $3;")
            .bind(id.as_str())
            .bind(avatar_json.clone().get())
            .bind(avatar_json.get())
            .execute(conn)
            .await
            .map_err(|e| {
                println!("Failed to insert avatar: {:?}", e);

                InsertError::Unknown
            })?;

        Ok(id)
    }
}
