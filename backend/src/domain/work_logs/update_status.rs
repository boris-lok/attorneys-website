use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::entity::WorkLogMappingStatus;
use crate::domain::work_log_mapping::repository::WorkLogMappingRepository;
use crate::domain::work_logs::repository::WorkLogsRepository;
use uuid::Uuid;

pub struct Request {
    pub id: Uuid,
    pub user_id: UserID,
    pub status: WorkLogMappingStatus,
}

pub enum Error {
    NotFound,
    Unknown(String),
}

/// validate and check permission, if force is true, ignore the permission checking
///
/// - validate the work log exists.
async fn validate(repo: &mut impl WorkLogsRepository, id: &Uuid) -> Result<(), Error> {
    let is_exist = repo
        .is_work_log_exist(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    if !is_exist {
        return Err(Error::NotFound);
    }

    Ok(())
}

pub async fn execute(
    work_log_repo: &mut impl WorkLogsRepository,
    work_log_mapping_repo: &mut impl WorkLogMappingRepository,
    req: Request,
) -> Result<(), Error> {
    validate(work_log_repo, &req.id).await?;

    work_log_mapping_repo
        .update_status(&req.id, &req.user_id, req.status)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
