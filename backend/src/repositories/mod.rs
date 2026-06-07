pub use article_views_repository::{IArticleViewsRepository, SqlxArticleViewsRepository};
pub use content_repository::{
    IContentRepository, InMemoryContentRepository, SqlxContentRepository,
};
pub use resource_repository::{
    IResourceRepository, InMemoryResourceRepository, SqlxResourceRepository,
};

#[cfg(test)]
pub use article_views_repository::InMemoryArticleViewsRepository;

use sqlx::{Pool, Postgres, Transaction};
use std::sync::Weak;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Connection<'tx> {
    Pool(Pool<Postgres>),
    Transaction(Weak<Mutex<Transaction<'tx, Postgres>>>),
}

mod article_views_repository;
mod content_repository;
mod resource_repository;
