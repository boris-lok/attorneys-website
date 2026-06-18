use crate::domain::cases::entity::CaseID;
use crate::domain::uow::common::Query;
use crate::domain::work_logs::entity::{WorkLog, WorkLogFilters};
use crate::domain::work_logs::error::WorkLogError;
use crate::domain::work_logs::repository::WorkLogsReadRepository;

#[derive(Debug)]
pub struct Request {
    pub case_id: CaseID,
    pub filters: WorkLogFilters,
}

pub async fn execute(query: &impl Query, req: Request) -> Result<Vec<WorkLog>, WorkLogError> {
    let res = query
        .work_log_repo()
        .list(&req.case_id, &req.filters)
        .await
        .map_err(|e| WorkLogError::Unknown(e.to_string()))?;

    Ok(res)
}
