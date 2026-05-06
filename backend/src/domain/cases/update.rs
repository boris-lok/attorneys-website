use crate::repositories::{CaseID, ICaseRepository};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Request {
    pub id: String,
    pub name: Option<String>,
    pub estimated_minutes: Option<i32>,
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl ICaseRepository + Sync + Send>>,
    req: Request,
) -> Result<(), Error> {
    let id = CaseID::try_from(req.id).map_err(Error::Unknown)?;
    let mut lock = repo.lock().await;

    lock.update(id, req.name, req.estimated_minutes)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))
}
