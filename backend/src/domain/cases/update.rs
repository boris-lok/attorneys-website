use crate::domain::cases::entity::UpdateCaseRequest;
use crate::domain::cases::repository::CaseRepository;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl CaseRepository + Sync + Send>>,
    req: UpdateCaseRequest,
) -> Result<(), Error> {
    let mut lock = repo.lock().await;

    lock.update(req)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))
}
