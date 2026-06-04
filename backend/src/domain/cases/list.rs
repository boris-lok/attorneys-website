use crate::domain::cases::entity::Case;
use crate::domain::cases::repository::CaseRepository;
use crate::domain::entities::UserID;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl CaseRepository + Sync + Send>>,
    id: &UserID,
) -> Result<Vec<Case>, Error> {
    let mut lock = repo.lock().await;

    let cases = lock
        .list(&id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(cases)
}
