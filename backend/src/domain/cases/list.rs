use crate::repositories::{Case, ICaseRepository};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl ICaseRepository + Sync + Send>>,
) -> Result<Vec<Case>, Error> {
    let lock = repo.lock().await;
    let cases = lock
        .list_cases()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(cases)
}
