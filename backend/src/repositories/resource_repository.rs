use crate::domain::entities::{ResourceID, ResourceType};
use anyhow::anyhow;
use sqlx::{Postgres, Transaction};
use std::sync::Weak;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IResourceRepository {
    async fn insert(
        &self,
        id: ResourceID,
        resource_type: ResourceType,
    ) -> anyhow::Result<ResourceID>;
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
impl<'tx> IResourceRepository for SqlxResourceRepository<'tx> {
    async fn insert(
        &self,
        id: ResourceID,
        resource_type: ResourceType,
    ) -> anyhow::Result<ResourceID> {
        todo!()
    }
}