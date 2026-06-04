use crate::domain::cases::entity::{CaseID, CreateCaseRequest};
use crate::domain::cases::repository::CaseRepository;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl CaseRepository + Sync + Send>>,
    req: CreateCaseRequest,
) -> Result<CaseID, Error> {
    let mut lock = repo.lock().await;

    let id = lock
        .create(req)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(id)
}
