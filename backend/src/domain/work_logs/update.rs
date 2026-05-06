use crate::domain::entities::UserID;
use crate::repositories::{IWorkLogsRepository, UpdateWorkLog};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub id: String,
    pub user_id: UserID,
    pub description: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    /// force update, ignore the following checking
    ///
    /// - check the work_log's creator is an owner.
    /// - check the work_log is co-operate with others.
    pub force: bool,
}

#[derive(Debug)]
pub enum Error {
    NotFound,
    PermissionDenied,
    InvalidID,
    InvalidStatus(String),
    Unknown(String),
}

/// validate and check permission, if force is true, ignore the permission checking
///
/// - validate the work log exists.
/// - check if the creator is an owner.
/// - check the work_log is co-operate with others.
async fn validate(
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

        let is_collaborator = lock
            .is_collaborator_work_log(id, user_id)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        if !is_creator || is_collaborator {
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

    validate(repo.clone(), &work_log_id, &req.user_id, req.force).await?;

    let mut lock = repo.lock().await;
    let req = UpdateWorkLog {
        id: work_log_id,
        description: req.description,
        started_at: req.started_at,
        ended_at: req.ended_at,
        deleted_at: None,
    };

    lock.update(req)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
