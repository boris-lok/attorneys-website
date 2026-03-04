use crate::repositories::{CaseID, IWorkLogsRepository, WorkLog};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Request {
    pub case_id: String,
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
        .list_work_logs(&case_id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(res)
}
