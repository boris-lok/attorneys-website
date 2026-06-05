use crate::domain::cases::entity::UpdateCaseRequest;
use crate::domain::cases::repository::CaseRepository;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(repo: &mut impl CaseRepository, req: UpdateCaseRequest) -> Result<(), Error> {
    repo.update(req)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))
}
