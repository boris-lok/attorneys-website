use crate::domain::entities::MemberID;
use anyhow::anyhow;
use sqlx::{Acquire, Postgres, Row, Transaction};
use std::sync::Weak;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IMemberRepository {
    async fn insert(&self, member_id: MemberID) -> anyhow::Result<MemberID>;
    async fn contains(&self, member_id: &MemberID) -> anyhow::Result<bool>;
    async fn delete(&self, member_id: MemberID) -> anyhow::Result<bool>;
}

#[derive(Debug)]
pub struct InMemoryMemberRepository {
    error: bool,
    members: Mutex<Vec<String>>,
}

impl InMemoryMemberRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            members: Mutex::new(Vec::new()),
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }

    pub async fn get(&self, id: &MemberID) -> anyhow::Result<Option<String>> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let lock = self.members.lock().await;

        Ok(lock
            .iter()
            .find(|e| e.as_str() == id.as_str())
            .map(|e| e.to_owned()))
    }
}

#[async_trait::async_trait]
impl IMemberRepository for InMemoryMemberRepository {
    async fn insert(&self, member_id: MemberID) -> anyhow::Result<MemberID> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.members.lock().await;

        if lock.iter().any(|id| id == member_id.as_str()) {
            return Err(anyhow!("{} already exists", member_id));
        }

        lock.push(member_id.to_string());
        Ok(member_id)
    }

    async fn contains(&self, member_id: &MemberID) -> anyhow::Result<bool> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let m_id = &member_id.as_str();
        let lock = self.members.lock().await;
        Ok(lock.iter().any(|id| id == m_id))
    }

    async fn delete(&self, member_id: MemberID) -> anyhow::Result<bool> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.members.lock().await;
        let removed = lock.iter().position(|id| id == member_id.as_str());
        match removed {
            Some(index) => {
                lock.remove(index);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

#[derive(Debug)]
pub struct SqlxMemberRepository<'tx> {
    tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
}

impl<'tx> SqlxMemberRepository<'tx> {
    pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl<'tx> IMemberRepository for SqlxMemberRepository<'tx> {
    async fn insert(&self, member_id: MemberID) -> anyhow::Result<MemberID> {
        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        sqlx::query("INSERT INTO \"member\" (id, created_at) VALUES ($1, now());")
            .bind(member_id.as_str())
            .execute(conn)
            .await?;

        Ok(member_id)
    }

    async fn contains(&self, id: &MemberID) -> anyhow::Result<bool> {
        let query = "SELECT id FROM \"member\" WHERE id = $1 limit 1";

        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        let res = sqlx::query(query)
            .bind(id.as_str())
            .fetch_optional(conn)
            .await
            .map(|row| match row {
                None => false,
                Some(row) => row.len() > 0,
            })?;

        Ok(res)
    }

    async fn delete(&self, member_id: MemberID) -> anyhow::Result<bool> {
        todo!()
    }
}
