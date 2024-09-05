use crate::domain::entities::{Member, MemberID};
use sqlx::{Acquire, Postgres, Transaction};
use std::sync::Weak;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum InsertError {
    Conflict,
    Unknown,
}

#[async_trait::async_trait]
pub trait IMemberRepository {
    async fn insert(&self, member_id: MemberID) -> Result<MemberID, InsertError>;
}

#[derive(Debug)]
pub struct InMemoryMemberRepository {
    error: bool,
    members: Mutex<Vec<Member>>,
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
}

#[async_trait::async_trait]
impl IMemberRepository for InMemoryMemberRepository {
    async fn insert(&self, member_id: MemberID) -> Result<MemberID, InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut lock = self.members.lock().await;

        if lock.iter().any(|m| m.member_id == member_id) {
            return Err(InsertError::Conflict);
        }

        let member_id_cloned = member_id.clone();
        lock.push(Member::new(member_id_cloned));
        Ok(member_id)
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
    async fn insert(&self, member_id: MemberID) -> Result<MemberID, InsertError> {
        let conn_ptr = self.tx.upgrade().ok_or(InsertError::Unknown)?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await.unwrap();

        sqlx::query("INSERT INTO \"member\" (id, created_at) VALUES ($1, now());")
            .bind(member_id.0.clone())
            .execute(conn)
            .await
            .map_err(|_| InsertError::Unknown)?;

        Ok(member_id)
    }
}
