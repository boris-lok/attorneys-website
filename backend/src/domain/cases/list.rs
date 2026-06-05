use crate::domain::cases::entity::Case;
use crate::domain::cases::repository::CaseRepository;
use crate::domain::entities::UserID;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(repo: &mut impl CaseRepository, id: &UserID) -> Result<Vec<Case>, Error> {
    let cases = repo
        .list(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(cases)
}
