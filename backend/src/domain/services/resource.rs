use crate::domain::resources::entity::{
    ContentID, CreateResourceRequest, ResourceID, UpdateResourceRequest,
};
use crate::domain::resources::repository::{ContentWriteRepository, ResourceWriteRepository};
use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::impl_uow;

impl_uow!(ResourceUoW);

impl<F: UnitOfWorkFactory> ResourceUoW<F> {
    pub async fn create(&self, req: CreateResourceRequest) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        async {
            uow.resource_repo()
                .create(&req.id, &req.kind, req.seq)
                .await?;

            let content_id = ContentID::from(req.id);

            uow.content_repo()
                .create(&content_id, req.data, req.language)
                .await
        }
        .await?;

        uow.commit().await?;

        Ok(())
    }

    pub async fn delete(&self, resource_id: &ResourceID) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;
        uow.resource_repo().delete(resource_id).await?;

        uow.commit().await?;

        Ok(())
    }

    pub async fn update(&self, req: UpdateResourceRequest) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        async {
            uow.resource_repo().update_seq(&req.id, req.seq).await?;

            uow.content_repo()
                .update(&ContentID::from(req.id), req.data, req.language)
                .await
        }
        .await?;

        uow.commit().await?;

        Ok(())
    }
}
