use crate::domain::entities::UserID;
use crate::repositories::IWorkLogsRepository;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub id: String,
    pub user_id: String,
    pub force: bool,
}

#[derive(Debug)]
pub enum Error {
    InvalidID,
    NotFound,
    PermissionDenied,
    Unknown(String),
}

/// validate and check permission, if force is true, ignore the permission checking
///
/// - validate the work log exists.
/// - check if the creator is an owner.
/// - check the work_log is co-operate with others.
async fn validate_and_check_permission(
    repo: Arc<Mutex<impl IWorkLogsRepository + Send + Sync>>,
    id: &Uuid,
    user_id: &UserID,
    force: bool,
) -> Result<(), Error> {
    let lock = repo.lock().await;

    let is_exist = lock
        .is_work_log_exist(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    if !is_exist {
        return Err(Error::NotFound);
    }

    if !force {
        let is_creator = lock
            .is_creator(id, user_id)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        if !is_creator {
            return Err(Error::PermissionDenied);
        }
    }

    Ok(())
}

pub async fn execute(
    repo: Arc<Mutex<impl IWorkLogsRepository + Send + Sync>>,
    req: Request,
) -> Result<(), Error> {
    let work_log_id = Uuid::parse_str(&req.id).map_err(|_| Error::InvalidID)?;
    let user_id =
        UserID::try_from(req.user_id).map_err(|_| Error::Unknown("Invalid user id".to_string()))?;

    validate_and_check_permission(repo.clone(), &work_log_id, &user_id, req.force).await?;

    let mut lock = repo.lock().await;

    lock.delete_work_log(work_log_id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
