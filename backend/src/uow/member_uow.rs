use crate::repositories::content_repository::{
    IContentRepository, InMemoryContentRepository, SqlxContentRepository,
};
use crate::repositories::member_repository::{
    IMemberRepository, InMemoryMemberRepository, SqlxMemberRepository,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IMemberUnitOfWork {
    type MemberRepo: IMemberRepository;
    type ContentRepo: IContentRepository;

    fn member_repository(&mut self) -> &mut Self::MemberRepo;
    fn content_repository(&mut self) -> &mut Self::ContentRepo;

    async fn commit(self) -> anyhow::Result<()>;
    async fn rollback(self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct InMemoryMemberUnitOfWork {
    error: bool,
    member_repository: Option<InMemoryMemberRepository>,
    content_repository: Option<InMemoryContentRepository>,
}

impl InMemoryMemberUnitOfWork {
    pub fn new() -> Self {
        Self {
            error: false,
            member_repository: None,
            content_repository: None,
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
impl IMemberUnitOfWork for InMemoryMemberUnitOfWork {
    type MemberRepo = InMemoryMemberRepository;
    type ContentRepo = InMemoryContentRepository;

    fn member_repository(&mut self) -> &mut Self::MemberRepo {
        if self.member_repository.is_none() {
            let member_repo = if self.error {
                InMemoryMemberRepository::new().with_error()
            } else {
                InMemoryMemberRepository::new()
            };
            self.member_repository = Some(member_repo);
        }
        self.member_repository.as_mut().unwrap()
    }

    fn content_repository(&mut self) -> &mut Self::ContentRepo {
        if self.content_repository.is_none() {
            let content_repo = if self.error {
                InMemoryContentRepository::new().with_error()
            } else {
                InMemoryContentRepository::new()
            };
            self.content_repository = Some(content_repo);
        }
        self.content_repository.as_mut().unwrap()
    }

    async fn commit(self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn rollback(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SqlxMemberUnitOfWork<'tx> {
    tx: Arc<Mutex<Transaction<'tx, Postgres>>>,
    member_repository: Option<SqlxMemberRepository<'tx>>,
}

impl<'tx> SqlxMemberUnitOfWork<'tx> {
    pub async fn new(pool: &PgPool) -> anyhow::Result<Self> {
        let tx = pool.begin().await?;
        let tx = Arc::new(Mutex::new(tx));

        Ok(Self {
            tx,
            member_repository: None,
        })
    }
}

#[async_trait::async_trait]
impl<'tx> IMemberUnitOfWork for SqlxMemberUnitOfWork<'tx> {
    type MemberRepo = SqlxMemberRepository<'tx>;
    type ContentRepo = SqlxContentRepository;

    fn member_repository(&mut self) -> &mut Self::MemberRepo {
        if self.member_repository.is_none() {
            let member_repo = SqlxMemberRepository::new(self.tx.clone());
            self.member_repository = Some(member_repo);
        }
        self.member_repository.as_mut().unwrap()
    }

    fn content_repository(&mut self) -> &mut Self::ContentRepo {
        todo!()
    }

    async fn commit(self) -> anyhow::Result<()> {
        let lock = Arc::try_unwrap(self.tx).unwrap();
        lock.into_inner().commit().await?;

        Ok(())
    }

    async fn rollback(self) -> anyhow::Result<()> {
        let lock = Arc::try_unwrap(self.tx).unwrap();
        lock.into_inner().rollback().await?;
        Ok(())
    }
}
