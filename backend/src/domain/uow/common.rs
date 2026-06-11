use crate::domain::articles::repository::ArticleViewWriteRepository;
use crate::domain::cases::repository::{CaseReadRepository, CaseRepository};
use crate::domain::resources::repository::{
    ContentWriteRepository, ResourceReadRepository, ResourceRepository,
};
use crate::domain::users::repository::{
    AvatarWriteRepository, UserReadRepository, UserRepository, UserRoleReadRepository,
    UserRoleRepository,
};
use crate::domain::work_log_mapping::repository::WorkLogMappingRepository;
use crate::domain::work_logs::repository::{WorkLogsReadRepository, WorkLogsRepository};

#[async_trait::async_trait]
pub trait UnitOfWork {
    type WorkLogRepo<'a>: WorkLogsRepository
    where
        Self: 'a;
    type WorkLogMappingRepo<'a>: WorkLogMappingRepository
    where
        Self: 'a;
    type UserRepo<'a>: UserRepository
    where
        Self: 'a;
    type UserRoleRepo<'a>: UserRoleRepository
    where
        Self: 'a;
    type AvatarRepo<'a>: AvatarWriteRepository
    where
        Self: 'a;
    type ArticleViewRepo<'a>: ArticleViewWriteRepository
    where
        Self: 'a;
    type ContentRepo<'a>: ContentWriteRepository
    where
        Self: 'a;
    type ResourceRepo<'a>: ResourceRepository
    where
        Self: 'a;
    type CaseRepo<'a>: CaseRepository
    where
        Self: 'a;
    type ArticleViewWriteRepo<'a>: ArticleViewWriteRepository
    where
        Self: 'a;

    fn work_log_repo(&mut self) -> Self::WorkLogRepo<'_>;
    fn work_log_mapping_repo(&mut self) -> Self::WorkLogMappingRepo<'_>;
    fn user_repo(&mut self) -> Self::UserRepo<'_>;
    fn user_role_repo(&mut self) -> Self::UserRoleRepo<'_>;
    fn avatar_repo(&mut self) -> Self::AvatarRepo<'_>;
    fn article_repo(&mut self) -> Self::ArticleViewRepo<'_>;
    fn content_repo(&mut self) -> Self::ContentRepo<'_>;
    fn resource_repo(&mut self) -> Self::ResourceRepo<'_>;
    fn case_repo(&mut self) -> Self::CaseRepo<'_>;
    fn article_view_repo(&mut self) -> Self::ArticleViewWriteRepo<'_>;
    async fn commit(self) -> anyhow::Result<()>;
    async fn rollback(self) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait UnitOfWorkFactory {
    type UoW: UnitOfWork;

    async fn begin(&self) -> anyhow::Result<Self::UoW>;
}

pub trait Query {
    type ResourceRepo<'a>: ResourceReadRepository
    where
        Self: 'a;
    type CaseRepo<'a>: CaseReadRepository
    where
        Self: 'a;
    type WorkLogRepo<'a>: WorkLogsReadRepository
    where
        Self: 'a;
    type UserRepo<'a>: UserReadRepository
    where
        Self: 'a;
    type UserRoleRepo<'a>: UserRoleReadRepository
    where
        Self: 'a;

    fn resource_repo(&self) -> Self::ResourceRepo<'_>;
    fn case_repo(&self) -> Self::CaseRepo<'_>;
    fn work_log_repo(&self) -> Self::WorkLogRepo<'_>;
    fn user_repo(&self) -> Self::UserRepo<'_>;
    fn user_role_repo(&self) -> Self::UserRoleRepo<'_>;
}

#[macro_export]
macro_rules! impl_uow {
    ($service:ident) => {
        pub struct $service<F: UnitOfWorkFactory> {
            factory: F,
        }

        impl<F: UnitOfWorkFactory> $service<F> {
            pub fn new(factory: F) -> Self {
                Self { factory }
            }
        }
    };
}
