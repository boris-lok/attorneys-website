use crate::domain::cases::entity::CaseID;
use crate::repositories::{IWorkLogsRepository, WorkLog};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Request {
    pub case_id: String,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub include_settled: bool,
}

#[derive(Debug)]
pub enum Error {
    InvalidCaseID,
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl IWorkLogsRepository + Sync + Send>>,
    req: Request,
) -> Result<Vec<WorkLog>, Error> {
    let case_id = CaseID::try_from(req.case_id).map_err(|_| Error::InvalidCaseID)?;

    let lock = repo.lock().await;

    let res = lock
        .list(&case_id, req.started_at, req.ended_at, req.include_settled)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(res)
}
