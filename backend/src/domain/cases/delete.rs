use crate::domain::cases::entity::CaseID;
use crate::domain::cases::repository::CaseRepository;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl CaseRepository + Sync + Send>>,
    id: &CaseID,
) -> Result<(), Error> {
    let mut repo = repo.lock().await;

    repo.delete(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
