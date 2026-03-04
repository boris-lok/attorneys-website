pub use article_views_repository::{IArticleViewsRepository, SqlxArticleViewsRepository};
pub use avatar_repository::{IAvatarRepository, InMemoryAvatarRepository, SqlxAvatarRepository};
pub use case_repository::{Case, CaseID, ICaseRepository, SQLxCaseRepository};
pub use content_repository::{
    IContentRepository, InMemoryContentRepository, SqlxContentRepository,
};
pub use resource_repository::{
    IResourceRepository, InMemoryResourceRepository, SqlxResourceRepository,
};
pub use roles_repository::{IRolesRepository, SqlxRolesRepository};
pub use user_repository::{IUserRepository, SqlxUserRepository, User};
pub use user_roles_repository::{IUserRolesRepository, SqlxUserRolesRepository};
pub use work_logs_repository::{
    CreateWorkLog, IWorkLogsRepository, SqlxWorkLogsRepository, WorkLog, WorkLogStatus,
};

#[cfg(test)]
pub use {
    article_views_repository::InMemoryArticleViewsRepository,
    user_repository::InMemoryUserRepository,
};

use sqlx::{Pool, Postgres, Transaction};
use std::sync::Weak;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Connection<'tx> {
    Pool(Pool<Postgres>),
    Transaction(Weak<Mutex<Transaction<'tx, Postgres>>>),
}

mod article_views_repository;
mod avatar_repository;
mod case_repository;
mod content_repository;
mod resource_repository;
mod roles_repository;
mod user_repository;
mod user_roles_repository;
mod work_logs_repository;
