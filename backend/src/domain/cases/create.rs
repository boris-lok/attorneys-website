use crate::repositories::ICaseRepository;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Request {
    pub name: String,
    pub estimated_minutes: i32,
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    req: Request,
    repo: Arc<Mutex<impl ICaseRepository + Sync + Send>>,
) -> Result<String, Error> {
    let mut lock = repo.lock().await;

    let id = lock
        .create_case(&req.name, req.estimated_minutes)
        .await
        .map_err(|e| {
            Error::Unknown(format!(
                "failed to create case {}, got an error: {}",
                req.name, e
            ))
        })?;

    Ok(id.into())
}
