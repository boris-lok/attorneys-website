use crate::domain::entities::UserID;
use crate::repositories::{IWorkLogsRepository, WorkLogStatus};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct Request {
    pub id: String,
    pub user_id: String,
    pub status: String,
}

pub enum Error {
    InvalidID,
    NotFound,
    InvalidStatus,
    Unknown(String),
}

/// validate and check permission, if force is true, ignore the permission checking
///
/// - validate the work log exists.
async fn validate(
    repo: Arc<Mutex<impl IWorkLogsRepository + Send + Sync>>,
    id: &Uuid,
) -> Result<(), Error> {
    let lock = repo.lock().await;

    let is_exist = lock
        .is_work_log_exist(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    if !is_exist {
        return Err(Error::NotFound);
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

    let status = WorkLogStatus::try_from(req.status).map_err(|_| Error::InvalidStatus)?;

    validate(repo.clone(), &work_log_id).await?;

    let mut lock = repo.lock().await;

    lock.update_status(&work_log_id, &user_id, status)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
