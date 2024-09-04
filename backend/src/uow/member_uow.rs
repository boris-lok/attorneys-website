use crate::repositories::content_repository::{IContentRepository, InMemoryContentRepository};
use crate::repositories::member_repository::{IMemberRepository, InMemoryMemberRepository};
use std::sync::Arc;

pub trait IUnitOfWork {
    type MemberRepo: IMemberRepository;
    type ContentRepo: IContentRepository;

    fn member_repository(&self) -> Arc<Self::MemberRepo>;
    fn content_repository(&self) -> Arc<Self::ContentRepo>;

    fn begin(&mut self) -> anyhow::Result<()>;
    fn commit(&self) -> anyhow::Result<()>;
    fn rollback(&self) -> anyhow::Result<()>;
}

pub struct InMemoryUnitOfWork {
    error: bool,
    member_repository: Option<Arc<InMemoryMemberRepository>>,
    content_repository: Option<Arc<InMemoryContentRepository>>,
}

impl InMemoryUnitOfWork {
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

impl IUnitOfWork for InMemoryUnitOfWork {
    type MemberRepo = InMemoryMemberRepository;
    type ContentRepo = InMemoryContentRepository;

    fn member_repository(&self) -> Arc<Self::MemberRepo> {
        self.member_repository.clone().unwrap()
    }

    fn content_repository(&self) -> Arc<Self::ContentRepo> {
        self.content_repository.clone().unwrap()
    }

    fn begin(&mut self) -> anyhow::Result<()> {
        let member_repo = if self.error {
            InMemoryMemberRepository::new().with_error()
        } else {
            InMemoryMemberRepository::new()
        };
        let content_repo = if self.error {
            InMemoryContentRepository::new().with_error()
        } else {
            InMemoryContentRepository::new()
        };
        self.member_repository = Some(Arc::new(member_repo));
        self.content_repository = Some(Arc::new(content_repo));
        Ok(())
    }

    fn commit(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn rollback(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
