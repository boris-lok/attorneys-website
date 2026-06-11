use crate::domain::cases::entity::Case;
use crate::domain::cases::repository::CaseReadRepository;
use crate::domain::uow::common::Query;
use crate::domain::users::entity::UserID;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(query: &impl Query, id: &UserID) -> Result<Vec<Case>, Error> {
    let cases = query
        .case_repo()
        .list(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(cases)
}
