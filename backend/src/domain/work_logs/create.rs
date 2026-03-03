use crate::domain::entities::UserID;
use crate::repositories::{CaseID, IWorkLogsRepository, WorkLog, WorkLogStatus};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub creator_id: UserID,
    pub case_id: Uuid,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub description: String,
    pub collaborators: Option<Vec<UserID>>,
}

impl TryFrom<Request> for WorkLog {
    type Error = String;

    fn try_from(value: Request) -> Result<Self, Self::Error> {
        let user_id = Uuid::parse_str(&value.creator_id.to_string()).unwrap();
        let case_id = CaseID::try_from(value.case_id.to_string())?;

        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            case_id,
            started_at: value.started_at,
            ended_at: value.started_at + value.duration,
            description: value.description,
            is_collaborative: value.collaborators.is_some(),
            parent_id: None,
            status: WorkLogStatus::Approved,
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
) -> Result<(), Error> {
    let collaborators = req.collaborators.clone().unwrap_or_default();

    let work_log = WorkLog::try_from(req).map_err(|_| Error::InvalidCaseID)?;
    let mut lock = repo.lock().await;

    lock.create_work_log(work_log.clone())
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    for collaborator in collaborators {
        let collaborator_work_log = create_a_collaborator_work_log(work_log.clone(), collaborator);
        lock.create_work_log(collaborator_work_log)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
    }

    Ok(())
}

fn create_a_collaborator_work_log(work_log: WorkLog, collaborator: UserID) -> WorkLog {
    WorkLog {
        parent_id: Some(work_log.id),
        id: Uuid::new_v4(),
        status: WorkLogStatus::Pending,
        user_id: Uuid::parse_str(&collaborator.to_string()).unwrap(),
        ..work_log
    }
}
