use crate::domain::entities::ResourceID;
use crate::domain::member::entities::AvatarJson;
use crate::repositories::Connection;
use anyhow::anyhow;
use sqlx::{Acquire, PgConnection};
use std::collections::HashMap;
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

impl Default for InMemoryAvatarRepository {
    fn default() -> Self {
        Self::new()
    }
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
    conn: Connection<'tx>,
}

impl<'tx> SqlxAvatarRepository<'tx> {
    pub fn new(conn: Connection<'tx>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl IAvatarRepository for SqlxAvatarRepository<'_> {
    async fn save(
        &self,
        id: ResourceID,
        avatar_json: AvatarJson,
    ) -> Result<ResourceID, InsertError> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await.map_err(|_| InsertError::Unknown)?;
                let conn = conn.as_mut();

                create(conn, id, avatar_json)
                    .await
                    .map_err(|_| InsertError::Unknown)
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(InsertError::Unknown)?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await.map_err(|_| InsertError::Unknown)?;

                create(conn, id, avatar_json)
                    .await
                    .map_err(|_| InsertError::Unknown)
            }
        }
    }
}

async fn create(
    conn: &mut PgConnection,
    id: ResourceID,
    avatar_json: AvatarJson,
) -> anyhow::Result<ResourceID> {
    sqlx::query("insert into \"avatar\" (id, data) values ($1, $2) on conflict (id) do update set data = $3;")
        .bind(id.as_str())
        .bind(avatar_json.clone().get())
        .bind(avatar_json.get())
        .execute(conn)
        .await?;

    Ok(id)
}
