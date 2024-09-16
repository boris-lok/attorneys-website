use crate::domain::service::entities::ServiceID;
use anyhow::anyhow;
use sqlx::{Acquire, Postgres, Transaction};
use std::sync::Weak;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IServiceRepository {
    async fn insert(&self, service_id: ServiceID) -> anyhow::Result<ServiceID>;
}

#[derive(Debug)]
pub struct InMemoryServiceRepository {
    error: bool,
    services: Mutex<Vec<String>>,
}

impl InMemoryServiceRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            services: Mutex::new(Vec::new()),
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }

    pub async fn get(&self, id: &ServiceID) -> anyhow::Result<Option<String>> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let lock = self.services.lock().await;

        Ok(lock
            .iter()
            .find(|e| e.as_str() == id.as_str())
            .map(|e| e.to_owned()))
    }
}

#[async_trait::async_trait]
impl IServiceRepository for InMemoryServiceRepository {
    async fn insert(&self, service_id: ServiceID) -> anyhow::Result<ServiceID> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.services.lock().await;

        if lock.iter().any(|id| id == service_id.as_str()) {
            return Err(anyhow!("{} already exists", service_id));
        }

        lock.push(service_id.to_string());
        Ok(service_id)
    }
}

#[derive(Debug)]
pub struct SqlxServiceRepository<'tx> {
    tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
}

impl<'tx> SqlxServiceRepository<'tx> {
    pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl<'tx> IServiceRepository for SqlxServiceRepository<'tx> {
    async fn insert(&self, service_id: ServiceID) -> anyhow::Result<ServiceID> {
        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        sqlx::query("INSERT INTO \"service\" (id, created_at, seq) VALUES ($1, now(), 32767 );")
            .bind(service_id.as_str())
            .execute(conn)
            .await?;

        Ok(service_id)
    }
}
