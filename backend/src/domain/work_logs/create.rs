use crate::domain::cases::entity::CaseID;
use crate::domain::entities::UserID;
use crate::repositories::{CreateWorkLog, IWorkLogsRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub creator_id: UserID,
    pub case_id: Uuid,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub description: String,
    pub collaborator_ids: Option<Vec<UserID>>,
}

impl TryFrom<Request> for CreateWorkLog {
    type Error = String;

    fn try_from(value: Request) -> Result<Self, Self::Error> {
        let user_id = Uuid::parse_str(&value.creator_id.to_string()).unwrap();
        let case_id = CaseID::try_from(value.case_id.to_string())?;
        let is_collaborative = value
            .collaborator_ids
            .map(|e| !e.is_empty())
            .unwrap_or(false);

        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            case_id,
            started_at: value.started_at,
            ended_at: value.started_at + value.duration,
            description: value.description,
            is_collaborative,
        })
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidCaseID,
    Unknown(String),
}

pub async fn execute(
    repo: Arc<tokio::sync::Mutex<impl IWorkLogsRepository + Sync + Send>>,
    req: Request,
) -> Result<Uuid, Error> {
    let collaborators = req.collaborator_ids.clone().unwrap_or_default();

    let work_log = CreateWorkLog::try_from(req).map_err(|_| Error::InvalidCaseID)?;
    let id = work_log.id;
    let mut lock = repo.lock().await;

    lock.create(work_log)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    if !collaborators.is_empty() {
        lock.create_mapping(id, collaborators)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
    }

    Ok(id)
}
