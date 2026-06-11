use crate::domain::services::work_log::WorkLogUoW;
use crate::domain::uow::common::UnitOfWorkFactory;
use crate::domain::users::entity::UserID;
use crate::domain::work_logs::error::WorkLogError;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub id: Uuid,
    pub user_id: UserID,
    pub force: bool,
}

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &WorkLogUoW<F>,
    req: Request,
) -> Result<(), WorkLogError> {
    uow.delete(&req.id, &req.user_id, req.force).await
}
