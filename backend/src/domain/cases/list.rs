use crate::domain::entities::UserID;
use crate::repositories::{Case, ICaseRepository};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Request {
    pub user_id: String,
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl ICaseRepository + Sync + Send>>,
    req: Request,
) -> Result<Vec<Case>, Error> {
    let user_id =
        UserID::try_from(req.user_id).map_err(|_| Error::Unknown("Invalid user id".to_string()))?;

    let lock = repo.lock().await;
    let cases = lock
        .list_cases(&user_id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(cases)
}
