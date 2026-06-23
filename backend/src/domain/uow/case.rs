use crate::domain::cases::entity::{CaseID, CreateCaseRequest, UpdateCaseRequest};
use crate::domain::cases::error::CaseError;
use crate::domain::cases::repository::{CaseRepository, CaseWriteRepository};
use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::domain::work_logs::repository::WorkLogsWriteRepository;
use crate::impl_uow;

impl_uow!(CaseUoW);

impl<F: UnitOfWorkFactory> CaseUoW<F> {
    pub async fn create(&self, req: CreateCaseRequest) -> Result<CaseID, CaseError> {
        let mut uow = self
            .factory
            .begin()
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))?;

        let id = uow
            .case_repo()
            .create(req)
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))?;

        uow.commit()
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))?;

        Ok(id)
    }

    pub async fn delete(&self, case_id: &CaseID) -> Result<(), CaseError> {
        let mut uow = self
            .factory
            .begin()
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))?;

        asset_case_is_not_closed_and_exist(&mut uow.case_repo(), case_id).await?;

        uow.case_repo()
            .delete(case_id)
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))?;

        uow.commit()
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))
    }

    pub async fn update(&self, req: UpdateCaseRequest) -> Result<(), CaseError> {
        let mut uow = self
            .factory
            .begin()
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))?;

        asset_case_is_not_closed_and_exist(&mut uow.case_repo(), &req.id).await?;

        uow.case_repo()
            .update(req)
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))?;

        uow.commit()
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))
    }

    pub async fn settle(&self, case_id: &CaseID) -> Result<(), CaseError> {
        let mut uow = self
            .factory
            .begin()
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))?;

        asset_case_is_not_closed_and_exist(&mut uow.case_repo(), case_id).await?;

        async {
            uow.case_repo().settle(case_id).await?;
            uow.work_log_repo().settle(case_id).await
        }
        .await
        .map_err(|e| CaseError::Unknown(e.to_string()))?;

        uow.commit()
            .await
            .map_err(|e| CaseError::Unknown(e.to_string()))
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
