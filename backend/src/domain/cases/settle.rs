use crate::repositories::{CaseID, ICaseRepository};
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
    req: Request,
    case_repo: Arc<Mutex<impl ICaseRepository + Sync + Send>>,
) -> Result<(), Error> {
    let case_id = CaseID::try_from(req.case_id).map_err(|_| Error::InvalidCaseID)?;

    let mut cr = case_repo.lock().await;
    cr.settle(&case_id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
