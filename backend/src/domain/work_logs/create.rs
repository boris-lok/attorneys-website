use crate::domain::cases::entity::CaseID;
use crate::domain::entities::UserID;
use crate::domain::services::work_log::WorkLogService;
use crate::domain::uow::common::UnitOfWorkFactory;
use crate::domain::work_log_mapping::repository::WorkLogMappingRepository;
use crate::domain::work_logs::entity::CreateWorkLogRequest;
use crate::domain::work_logs::repository::WorkLogsRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub id: Uuid,
    pub user_id: Uuid,
    pub case_id: CaseID,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub collaborator_ids: Vec<UserID>,
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute<F: UnitOfWorkFactory>(
    service: &WorkLogService<F>,
    req: Request,
) -> Result<Uuid, Error> {
    let id = req.id;

    let work_log = CreateWorkLogRequest {
        id,
        user_id: req.user_id,
        case_id: req.case_id,
        started_at: req.started_at,
        ended_at: req.ended_at,
        description: req.description,
        is_collaborative: !req.collaborator_ids.is_empty(),
    };

    service
        .create(work_log, req.collaborator_ids)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(id)
}
