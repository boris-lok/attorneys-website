use crate::domain::cases::entity::CaseID;
use crate::domain::cases::repository::CaseRepository;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(repo: &mut impl CaseRepository, id: &CaseID) -> Result<(), Error> {
    repo.settle(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
