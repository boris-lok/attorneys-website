use crate::domain::uow::common::UnitOfWorkFactory;
use crate::domain::uow::work_log::WorkLogUoW;
use crate::domain::users::entity::UserID;
use crate::domain::work_logs::entity::UpdateWorkLogRequest;
use crate::domain::work_logs::error::WorkLogError;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub id: Uuid,
    pub user_id: UserID,
    pub description: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    /// force update, ignore the following checking
    ///
    /// - check the work_log's creator is an owner.
    /// - check the work_log is co-operate with others.
    pub force: bool,
}

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &WorkLogUoW<F>,
    req: Request,
) -> Result<(), WorkLogError> {
    let user_id = &req.user_id;
    let force = req.force;
    let req = UpdateWorkLogRequest {
        id: req.id,
        description: req.description,
        started_at: req.started_at,
        ended_at: req.ended_at,
    };

    uow.update(req, user_id, force).await
}
