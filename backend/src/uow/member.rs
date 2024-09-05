use crate::repositories::content_repository::{
    IContentRepository, InMemoryContentRepository, SqlxContentRepository,
};
use crate::repositories::member_repository::{
    IMemberRepository, InMemoryMemberRepository, SqlxMemberRepository,
};
use anyhow::anyhow;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IMemberUnitOfWork {
    type MemberRepo: IMemberRepository;
    type ContentRepo: IContentRepository;

    fn member_repository(&mut self) -> &mut Self::MemberRepo;
    fn content_repository(&mut self) -> &mut Self::ContentRepo;

    async fn commit(mut self) -> anyhow::Result<()>;
    async fn rollback(mut self) -> anyhow::Result<()>;
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
    content_repository: Option<SqlxContentRepository<'tx>>,
}

impl<'tx> SqlxMemberUnitOfWork<'tx> {
    pub async fn new(pool: &PgPool) -> anyhow::Result<Self> {
        let tx = pool.begin().await?;
        let tx = Arc::new(Mutex::new(tx));

        Ok(Self {
            tx,
            member_repository: None,
            content_repository: None,
        })
    }
}

#[async_trait::async_trait]
impl<'tx> IMemberUnitOfWork for SqlxMemberUnitOfWork<'tx> {
    type MemberRepo = SqlxMemberRepository<'tx>;
    type ContentRepo = SqlxContentRepository<'tx>;

    fn member_repository(&mut self) -> &mut Self::MemberRepo {
        if self.member_repository.is_none() {
            let member_repo = SqlxMemberRepository::new(Arc::downgrade(&self.tx));
            self.member_repository = Some(member_repo);
        }
        self.member_repository.as_mut().unwrap()
    }

    fn content_repository(&mut self) -> &mut Self::ContentRepo {
        if self.content_repository.is_none() {
            // Use Arc::downgrade to obtain a weak reference to the transaction
            // if we don't do this, when we call the commit/rollback method will fail.
            // It can't `try_unwrap` because there are at least two strong references, preventing
            // the use of `try_unwrap`.
            //
            // If we want to use strong references, then we need to drop the repository
            // when we try to call commit/rollback methods.
            let content_repo = SqlxContentRepository::new(Arc::downgrade(&self.tx));
            self.content_repository = Some(content_repo);
        }
        self.content_repository.as_mut().unwrap()
    }

    async fn commit(mut self) -> anyhow::Result<()> {
        match Arc::try_unwrap(self.tx) {
            Ok(lock) => {
                lock.into_inner().commit().await?;
                Ok(())
            }
            Err(_) => Err(anyhow!("can't commit transaction")),
        }
    }

    async fn rollback(mut self) -> anyhow::Result<()> {
        match Arc::try_unwrap(self.tx) {
            Ok(lock) => {
                lock.into_inner().rollback().await?;
                Ok(())
            }
            Err(_) => Err(anyhow!("can't rollback transaction")),
        }
    }
}
