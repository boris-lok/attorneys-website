use crate::domain::articles::repository::ArticleViewWriteRepository;
use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::impl_uow;
use std::net::IpAddr;
use uuid::Uuid;

impl_uow!(ArticleViewUoW);

impl<F: UnitOfWorkFactory> ArticleViewUoW<F> {
    pub async fn create(&self, id: &str, ip: &IpAddr, user_agent: &str) -> anyhow::Result<Uuid> {
        let mut uow = self.factory.begin().await?;

        let id = uow.article_view_repo().create(id, ip, user_agent).await?;

        uow.commit().await?;

        Ok(id)
    }
}
