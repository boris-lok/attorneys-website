use crate::domain::services::work_log::WorkLogUoW;
use crate::domain::uow::common::UnitOfWorkFactory;
use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::entity::WorkLogMappingStatus;
use crate::domain::work_logs::error::WorkLogError;
use uuid::Uuid;

pub struct Request {
    pub id: Uuid,
    pub user_id: UserID,
    pub status: WorkLogMappingStatus,
}

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &WorkLogUoW<F>,
    req: Request,
) -> Result<(), WorkLogError> {
    uow.update_status(&req.id, &req.user_id, req.status).await
}
