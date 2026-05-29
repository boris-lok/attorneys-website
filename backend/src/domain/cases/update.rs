use crate::repositories::{CaseID, ICaseRepository};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Request {
    pub id: String,
    pub name: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub estimated_minutes: Option<i32>,
    pub billing_cycle: Option<i32>,
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

    lock.update(
        id,
        req.name,
        req.started_at,
        req.ended_at,
        req.estimated_minutes,
        req.billing_cycle,
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))
}
