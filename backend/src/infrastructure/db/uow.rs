use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::infrastructure::db::connection::{
    AvatarRepo, PostgresRepo, UserRepo, UserRoleRepo, WorkLogMappingRepo, WorkLogRepo,
};
use sqlx::PgPool;

pub struct PostgresUoWFactory {
    pool: PgPool,
}

impl PostgresUoWFactory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UnitOfWorkFactory for PostgresUoWFactory {
    type UoW = PostgresUoW;

    async fn begin(&self) -> anyhow::Result<Self::UoW> {
        let tx = self.pool.begin().await?;

        Ok(Self::UoW { tx })
    }
}

pub struct PostgresUoW {
    tx: sqlx::Transaction<'static, sqlx::Postgres>,
}

#[async_trait::async_trait]
impl UnitOfWork for PostgresUoW {
    type WorkLogRepo<'a> = PostgresRepo<'a, WorkLogRepo>;
    type WorkLogMappingRepo<'a> = PostgresRepo<'a, WorkLogMappingRepo>;
    type UserRepo<'a> = PostgresRepo<'a, UserRepo>;
    type UserRoleRepo<'a> = PostgresRepo<'a, UserRoleRepo>;
    type AvatarRepo<'a> = PostgresRepo<'a, AvatarRepo>;

    fn work_log_repo(&mut self) -> Self::WorkLogRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn work_log_mapping_repo(&mut self) -> Self::WorkLogMappingRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn user_repo(&mut self) -> Self::UserRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn user_role_repo(&mut self) -> Self::UserRoleRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn avatar_repo(&mut self) -> Self::AvatarRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    async fn commit(self) -> anyhow::Result<()> {
        self.tx.commit().await?;
        Ok(())
    }

    async fn rollback(self) -> anyhow::Result<()> {
        self.tx.rollback().await?;
        Ok(())
    }
}
