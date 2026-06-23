use crate::domain::cases::entity::{CaseID, CreateCaseRequest, UpdateCaseRequest};
use crate::domain::cases::error::CaseError;
use crate::domain::cases::repository::{CaseRepository, CaseWriteRepository};
use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::domain::work_logs::repository::WorkLogsWriteRepository;
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

        asset_case_is_not_closed_and_exist(&mut uow.case_repo(), case_id).await?;

        uow.case_repo().delete(case_id).await?;

        uow.commit().await
    }

    pub async fn update(&self, req: UpdateCaseRequest) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        asset_case_is_not_closed_and_exist(&mut uow.case_repo(), &req.id).await?;

        uow.case_repo().update(req).await?;

        uow.commit().await
    }

    pub async fn settle(&self, case_id: &CaseID) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        asset_case_is_not_closed_and_exist(&mut uow.case_repo(), case_id).await?;

        async {
            uow.case_repo().settle(case_id).await?;
            uow.work_log_repo().settle(case_id).await
        }
        .await?;

        uow.commit().await
    }
}

pub(crate) async fn asset_case_is_not_closed_and_exist(
    repo: &mut impl CaseRepository,
    id: &CaseID,
) -> Result<(), CaseError> {
    let case = repo
        .retrieve(id)
        .await
        .map_err(|e| CaseError::Unknown(e.to_string()))?;

    match case {
        None => Err(CaseError::NotFound),
        Some(c) if c.closed => Err(CaseError::CaseIsClosed),
        Some(_) => Ok(()),
    }
}
