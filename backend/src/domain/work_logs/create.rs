use crate::domain::entities::UserID;
use crate::repositories::{IWorkLogsRepository, WorkLog, WorkLogStatus};
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

impl From<Request> for WorkLog {
    fn from(value: Request) -> Self {
        let user_id = Uuid::parse_str(&value.creator_id.to_string()).unwrap();

        Self {
            id: Uuid::new_v4(),
            user_id,
            case_id: value.case_id,
            started_at: value.started_at,
            ended_at: value.started_at + value.duration,
            description: value.description,
            is_collaborative: value.collaborators.is_some(),
            parent_id: None,
            status: WorkLogStatus::Approved,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    req: Request,
    repo: tokio::sync::Mutex<impl IWorkLogsRepository + Sync + Send>,
) -> Result<(), Error> {
    let collaborators = req.collaborators.clone().unwrap_or_default();

    let work_log = WorkLog::from(req);

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
