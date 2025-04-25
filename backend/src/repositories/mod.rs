pub use avatar_repository::IAvatarRepository;
pub use avatar_repository::InMemoryAvatarRepository;
pub use avatar_repository::SqlxAvatarRepository;
use sqlx::{Pool, Postgres, Transaction};
use std::sync::Weak;
use tokio::sync::Mutex;

pub use content_repository::IContentRepository;
pub use content_repository::InMemoryContentRepository;
pub use content_repository::SqlxContentRepository;

pub use resource_repository::IResourceRepository;
pub use resource_repository::InMemoryResourceRepository;
pub use resource_repository::SqlxResourceRepository;

pub use user_repository::IUserRepository;
#[cfg(test)]
pub use user_repository::InMemoryUserRepository;
pub use user_repository::SqlxUserRepository;

pub use article_views_repository::IArticleViewsRepository;
#[cfg(test)]
pub use article_views_repository::InMemoryArticleViewsRepository;
pub use article_views_repository::SqlxArticleViewsRepository;

#[derive(Debug)]
pub enum Connection<'tx> {
    Pool(Pool<Postgres>),
    Transaction(Weak<Mutex<Transaction<'tx, Postgres>>>),
}

mod avatar_repository;

mod content_repository;

mod resource_repository;

mod user_repository;

mod article_views_repository;
