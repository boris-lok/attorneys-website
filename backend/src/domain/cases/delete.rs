use crate::repositories::{CaseID, ICaseRepository};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Request {
    pub id: String,
}

pub enum Error {
    InvalidID,
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl ICaseRepository + Sync + Send>>,
    req: Request,
) -> Result<(), Error> {
    let case_id = CaseID::try_from(req.id).map_err(|e| Error::InvalidID)?;

    let mut repo = repo.lock().await;

    repo.delete(case_id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
