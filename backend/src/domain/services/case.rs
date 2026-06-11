use crate::domain::cases::entity::{CaseID, CreateCaseRequest, UpdateCaseRequest};
use crate::domain::cases::repository::CaseWriteRepository;
use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::impl_uow;

impl_uow!(CaseUoW);

impl<F: UnitOfWorkFactory> CaseUoW<F> {
    pub async fn create(&self, req: CreateCaseRequest) -> anyhow::Result<CaseID> {
        let mut uow = self.factory.begin().await?;

        let id = uow.case_repo().create(req).await?;

        uow.commit().await?;

        Ok(id)
    }

    pub async fn delete(&self, case_id: &CaseID) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        uow.case_repo().delete(case_id).await?;

        uow.commit().await?;

        Ok(())
    }

    pub async fn update(&self, req: UpdateCaseRequest) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        uow.case_repo().update(req).await?;

        uow.commit().await?;

        Ok(())
    }

    pub async fn settle(&self, case_id: &CaseID) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        uow.case_repo().settle(case_id).await?;

        uow.commit().await?;

        Ok(())
    }
}
