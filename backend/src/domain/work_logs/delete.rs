use crate::repositories::IWorkLogsRepository;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub id: String,
}

#[derive(Debug)]
pub enum Error {
    InvalidID,
    Unknown(String),
}

pub async fn execute(
    repo: Arc<Mutex<impl IWorkLogsRepository + Send + Sync>>,
    req: Request,
) -> Result<(), Error> {
    let work_log_id = Uuid::parse_str(&req.id).map_err(|_| Error::InvalidID)?;

    let mut lock = repo.lock().await;

    lock.delete_work_log(work_log_id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
