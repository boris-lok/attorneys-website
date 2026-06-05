use crate::domain::users::repository::{UserRepository, UserRoleRepository};
use crate::domain::work_log_mapping::repository::WorkLogMappingRepository;
use crate::domain::work_logs::repository::WorkLogsRepository;

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

    fn work_log_repo(&mut self) -> Self::WorkLogRepo<'_>;

    fn work_log_mapping_repo(&mut self) -> Self::WorkLogMappingRepo<'_>;

    fn user_repo(&mut self) -> Self::UserRepo<'_>;

    fn user_role_repo(&mut self) -> Self::UserRoleRepo<'_>;

    async fn commit(self) -> anyhow::Result<()>;

    async fn rollback(self) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait UnitOfWorkFactory {
    type UoW: UnitOfWork;

    async fn begin(&self) -> anyhow::Result<Self::UoW>;
}
