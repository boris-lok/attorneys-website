use crate::domain::cases::entity::{CaseID, CreateCaseRequest};
use crate::domain::cases::repository::CaseRepository;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: &mut impl CaseRepository,
    req: CreateCaseRequest,
) -> Result<CaseID, Error> {
    let id = repo
        .create(req)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(id)
}
