use crate::domain::entities::UserID;
use crate::domain::work_logs::repository::WorkLogsRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub id: Uuid,
    pub user_id: UserID,
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
async fn validate(
    repo: &mut impl WorkLogsRepository,
    id: &Uuid,
    user_id: &UserID,
    force: bool,
) -> Result<(), Error> {
    let is_exist = repo
        .is_work_log_exist(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    if !is_exist {
        return Err(Error::NotFound);
    }

    if !force {
        let is_creator = repo
            .is_creator(id, user_id)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        if !is_creator {
            return Err(Error::PermissionDenied);
        }
    }

    Ok(())
}

pub async fn execute(repo: &mut impl WorkLogsRepository, req: Request) -> Result<(), Error> {
    validate(repo, &req.id, &req.user_id, req.force).await?;

    repo.delete(&req.id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
