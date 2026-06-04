use crate::domain::cases::entity::CaseID;
use crate::domain::cases::repository::CaseRepository;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    case_repo: Arc<Mutex<impl CaseRepository + Sync + Send>>,
    id: &CaseID,
) -> Result<(), Error> {
    let mut cr = case_repo.lock().await;

    cr.settle(&id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
