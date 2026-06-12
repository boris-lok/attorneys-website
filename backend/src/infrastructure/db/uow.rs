use crate::domain::uow::common::{Query, UnitOfWork, UnitOfWorkFactory};
use crate::infrastructure::db::connection::{
    ArticleViewRepo, AvatarRepo, CaseRepo, ContentRepo, PostgresRepo, ResourceRepo, RoleRepo,
    UserRepo, UserRoleRepo, WorkLogMappingRepo, WorkLogRepo,
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
    type ArticleViewRepo<'a> = PostgresRepo<'a, ArticleViewRepo>;
    type ContentRepo<'a> = PostgresRepo<'a, ContentRepo>;
    type ResourceRepo<'a> = PostgresRepo<'a, ResourceRepo>;
    type CaseRepo<'a> = PostgresRepo<'a, CaseRepo>;
    type ArticleViewWriteRepo<'a> = PostgresRepo<'a, ArticleViewRepo>;
    type RoleRepo<'a> = PostgresRepo<'a, RoleRepo>;

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

    fn article_repo(&mut self) -> Self::ArticleViewRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn content_repo(&mut self) -> Self::ContentRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn resource_repo(&mut self) -> Self::ResourceRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn case_repo(&mut self) -> Self::CaseRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn article_view_repo(&mut self) -> Self::ArticleViewRepo<'_> {
        PostgresRepo::with_tx(&mut self.tx)
    }

    fn role_repo(&mut self) -> Self::RoleRepo<'_> {
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

pub struct PostgresQuery {
    pool: PgPool,
}

impl PostgresQuery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Query for PostgresQuery {
    type ResourceRepo<'a> = PostgresRepo<'a, ResourceRepo>;
    type CaseRepo<'a> = PostgresRepo<'a, CaseRepo>;
    type WorkLogRepo<'a> = PostgresRepo<'a, WorkLogRepo>;
    type UserRepo<'a> = PostgresRepo<'a, UserRepo>;
    type UserRoleRepo<'a> = PostgresRepo<'a, UserRoleRepo>;
    type RoleRepo<'a> = PostgresRepo<'a, RoleRepo>;

    fn resource_repo(&self) -> Self::ResourceRepo<'_> {
        PostgresRepo::from_pool(&self.pool)
    }

    fn case_repo(&self) -> Self::CaseRepo<'_> {
        PostgresRepo::from_pool(&self.pool)
    }

    fn work_log_repo(&self) -> Self::WorkLogRepo<'_> {
        PostgresRepo::from_pool(&self.pool)
    }

    fn user_repo(&self) -> Self::UserRepo<'_> {
        PostgresRepo::from_pool(&self.pool)
    }

    fn user_role_repo(&self) -> Self::UserRoleRepo<'_> {
        PostgresRepo::from_pool(&self.pool)
    }

    fn role_repo(&self) -> Self::RoleRepo<'_> {
        PostgresRepo::from_pool(&self.pool)
    }
}
