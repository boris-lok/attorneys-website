use crate::domain::entities::{ResourceID, ResourceType};
use anyhow::anyhow;
use sqlx::{Acquire, Postgres, Row, Transaction};
use std::sync::Weak;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IResourceRepository {
    async fn insert(
        &self,
        id: ResourceID,
        resource_type: ResourceType,
        seq: i32,
    ) -> anyhow::Result<ResourceID>;

    // check if the resource is already in the repository
    async fn contains(&self, id: &ResourceID, resource_type: &ResourceType)
        -> anyhow::Result<bool>;

    // delete the resource from the repository
    async fn delete(&self, id: &ResourceID, resource_type: &ResourceType) -> anyhow::Result<()>;

    // update the sequence of the resource in the repository
    async fn update_seq(&self, id: &ResourceID, seq: i32) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct InMemoryResourceRepository {
    error: bool,
    resources: Mutex<Vec<(ResourceID, ResourceType)>>,
}

impl InMemoryResourceRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            resources: Mutex::new(Vec::new()),
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
impl IResourceRepository for InMemoryResourceRepository {
    async fn insert(
        &self,
        id: ResourceID,
        resource_type: ResourceType,
        _: i32,
    ) -> anyhow::Result<ResourceID> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.resources.lock().await;

        if lock
            .iter()
            .any(|(res_id, kind)| res_id == &id && kind == &resource_type)
        {
            return Err(anyhow!("{} already exists", id));
        }

        lock.push((id.clone(), resource_type));

        Ok(id)
    }

    async fn contains(
        &self,
        id: &ResourceID,
        resource_type: &ResourceType,
    ) -> anyhow::Result<bool> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let lock = self.resources.lock().await;

        Ok(lock
            .iter()
            .any(|(res_id, kind)| res_id == id && kind == resource_type))
    }

    async fn delete(&self, id: &ResourceID, resource_type: &ResourceType) -> anyhow::Result<()> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.resources.lock().await;

        let removed = lock
            .iter()
            .position(|(res_id, kind)| res_id == id && kind == resource_type);
        match removed {
            Some(index) => {
                lock.remove(index);
                Ok(())
            }
            None => Err(anyhow!("{} not found", id)),
        }
    }

    async fn update_seq(&self, _: &ResourceID, _: i32) -> anyhow::Result<()> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        // TODO: handle seq

        Ok(())
    }
}

#[derive(Debug)]
pub struct SqlxResourceRepository<'tx> {
    tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
}

impl<'tx> SqlxResourceRepository<'tx> {
    pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl IResourceRepository for SqlxResourceRepository<'_> {
    async fn insert(
        &self,
        id: ResourceID,
        resource_type: ResourceType,
        seq: i32,
    ) -> anyhow::Result<ResourceID> {
        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        sqlx::query("INSERT INTO \"resource\" (id, created_at, resource_type, seq) VALUES ($1, now(), $2, $3);")
            .bind(id.as_str())
            .bind(resource_type.as_str())
            .bind(seq)
            .execute(conn)
            .await?;

        Ok(id)
    }

    async fn contains(
        &self,
        id: &ResourceID,
        resource_type: &ResourceType,
    ) -> anyhow::Result<bool> {
        let query = "SELECT id FROM \"resource\" WHERE id = $1 and resource_type = $2 limit 1";

        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        let res = sqlx::query(query)
            .bind(id.as_str())
            .bind(resource_type.as_str())
            .fetch_optional(conn)
            .await
            .map(|row| match row {
                None => false,
                Some(row) => row.len() > 0,
            })?;

        Ok(res)
    }

    async fn delete(&self, id: &ResourceID, resource_type: &ResourceType) -> anyhow::Result<()> {
        let query =
            "UPDATE \"resource\" SET deleted_at = now() WHERE id = $1 and resource_type = $2";

        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        sqlx::query(query)
            .bind(id.as_str())
            .bind(resource_type.as_str())
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn update_seq(&self, id: &ResourceID, seq: i32) -> anyhow::Result<()> {
        let query = "UPDATE \"resource\" SET seq = $2 WHERE id = $1";

        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        sqlx::query(query)
            .bind(id.as_str())
            .bind(seq)
            .execute(conn)
            .await?;

        Ok(())
    }
}
