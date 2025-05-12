use crate::domain::entities::{ResourceID, ResourceType};
use crate::repositories::Connection;
use anyhow::anyhow;
use sqlx::{Acquire, PgConnection, Row};
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

impl Default for InMemoryResourceRepository {
    fn default() -> Self {
        Self::new()
    }
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
    conn: Connection<'tx>,
}

impl<'tx> SqlxResourceRepository<'tx> {
    pub fn new(conn: Connection<'tx>) -> Self {
        Self { conn }
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
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                create(conn, id, resource_type, seq).await
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                create(conn, id, resource_type, seq).await
            }
        }
    }

    async fn contains(
        &self,
        id: &ResourceID,
        resource_type: &ResourceType,
    ) -> anyhow::Result<bool> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                contains(conn, id, resource_type).await
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                contains(conn, id, resource_type).await
            }
        }
    }

    async fn delete(&self, id: &ResourceID, resource_type: &ResourceType) -> anyhow::Result<()> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                delete(conn, id, resource_type).await
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                delete(conn, id, resource_type).await
            }
        }
    }

    async fn update_seq(&self, id: &ResourceID, seq: i32) -> anyhow::Result<()> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                update_seq(conn, id, seq).await
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                update_seq(conn, id, seq).await
            }
        }
    }
}

async fn create(
    conn: &mut PgConnection,
    id: ResourceID,
    resource_type: ResourceType,
    seq: i32,
) -> anyhow::Result<ResourceID> {
    sqlx::query(
        "INSERT INTO \"resource\" (id, created_at, resource_type, seq) VALUES ($1, now(), $2, $3);",
    )
    .bind(id.as_str())
    .bind(resource_type.as_str())
    .bind(seq)
    .execute(conn)
    .await?;

    Ok(id)
}

async fn contains(
    conn: &mut PgConnection,
    id: &ResourceID,
    resource_type: &ResourceType,
) -> anyhow::Result<bool> {
    let res =
        sqlx::query("SELECT id FROM \"resource\" WHERE id = $1 and resource_type = $2 limit 1;")
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

async fn delete(
    conn: &mut PgConnection,
    id: &ResourceID,
    resource_type: &ResourceType,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE \"resource\" SET deleted_at = now() WHERE id = $1 and resource_type = $2;")
        .bind(id.as_str())
        .bind(resource_type.as_str())
        .execute(conn)
        .await?;

    Ok(())
}

async fn update_seq(conn: &mut PgConnection, id: &ResourceID, seq: i32) -> anyhow::Result<()> {
    sqlx::query("UPDATE \"resource\" SET seq = $2 WHERE id = $1;")
        .bind(id.as_str())
        .bind(seq)
        .execute(conn)
        .await?;

    Ok(())
}
