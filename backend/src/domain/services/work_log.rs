use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::entity::WorkLogMappingStatus;
use crate::domain::work_log_mapping::repository::WorkLogMappingRepository;
use crate::domain::work_logs::entity::{CreateWorkLogRequest, UpdateWorkLogRequest};
use crate::domain::work_logs::error::WorkLogError;
use crate::domain::work_logs::repository::{WorkLogsRepository, WorkLogsWriteRepository};
use crate::impl_uow;
use uuid::Uuid;

impl_uow!(WorkLogUoW);

impl<F: UnitOfWorkFactory> WorkLogUoW<F> {
    pub async fn create(
        &self,
        log: CreateWorkLogRequest,
        collaborator_ids: Vec<UserID>,
    ) -> Result<(), WorkLogError> {
        let mut uow = self
            .factory
            .begin()
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        async {
            let id = log.id;
            uow.work_log_repo().create(log).await?;
            uow.work_log_mapping_repo()
                .create(&id, collaborator_ids)
                .await
        }
        .await
        .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        uow.commit()
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        Ok(())
    }

    pub async fn delete(
        &self,
        id: &Uuid,
        user_id: &UserID,
        force: bool,
    ) -> Result<(), WorkLogError> {
        let mut uow = self
            .factory
            .begin()
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        asset_exist(&mut uow.work_log_repo(), id).await?;
        if !force {
            asset_owner(&mut uow.work_log_repo(), id, user_id).await?;
        }

        uow.work_log_repo()
            .delete(id)
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        uow.commit()
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))
    }

    pub async fn update(
        &self,
        req: UpdateWorkLogRequest,
        user_id: &UserID,
        force: bool,
    ) -> Result<(), WorkLogError> {
        let mut uow = self
            .factory
            .begin()
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        asset_exist(&mut uow.work_log_repo(), &req.id).await?;

        if !force {
            asset_owner(&mut uow.work_log_repo(), &req.id, user_id).await?;
            asset_not_collaborator(&mut uow.work_log_repo(), &req.id, user_id).await?;
        }

        uow.work_log_repo()
            .update(req)
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        uow.commit()
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))
    }

    pub async fn update_status(
        &self,
        id: &Uuid,
        user_id: &UserID,
        status: WorkLogMappingStatus,
    ) -> Result<(), WorkLogError> {
        let mut uow = self
            .factory
            .begin()
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        asset_exist(&mut uow.work_log_repo(), id).await?;

        uow.work_log_mapping_repo()
            .update_status(id, user_id, status)
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

        uow.commit()
            .await
            .map_err(|e| WorkLogError::Unknown(e.to_string()))
    }
}

async fn asset_exist(repo: &mut impl WorkLogsRepository, id: &Uuid) -> Result<(), WorkLogError> {
    let is_exist = repo
        .is_work_log_exist(id)
        .await
        .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

    match is_exist {
        true => Ok(()),
        false => Err(WorkLogError::NotFound),
    }
}

async fn asset_owner(
    repo: &mut impl WorkLogsRepository,
    id: &Uuid,
    user_id: &UserID,
) -> Result<(), WorkLogError> {
    let is_owner = repo
        .is_creator(id, user_id)
        .await
        .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

    match is_owner {
        true => Ok(()),
        false => Err(WorkLogError::PermissionDenied),
    }
}

async fn asset_not_collaborator(
    repo: &mut impl WorkLogsRepository,
    id: &Uuid,
    user_id: &UserID,
) -> Result<(), WorkLogError> {
    let is_collaborator = repo
        .is_collaborator_work_log(&id, user_id)
        .await
        .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

    match is_collaborator {
        true => Err(WorkLogError::PermissionDenied),
        false => Ok(()),
    }
}
