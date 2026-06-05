use crate::domain::cases::entity::CaseID;
use crate::domain::work_logs::entity::WorkLog;
use crate::domain::work_logs::repository::WorkLogsRepository;

#[derive(Debug)]
pub struct Request {
    pub case_id: CaseID,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub include_settled: bool,
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: &mut impl WorkLogsRepository,
    req: Request,
) -> Result<Vec<WorkLog>, Error> {
    let res = repo
        .list(
            &req.case_id,
            req.started_at,
            req.ended_at,
            req.include_settled,
        )
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(res)
}
