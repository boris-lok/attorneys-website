use crate::domain::cases::entity::Case;
use crate::domain::cases::error::CaseError;
use crate::domain::cases::repository::CaseReadRepository;
use crate::domain::uow::common::Query;
use crate::domain::users::entity::UserID;

pub async fn execute(query: &impl Query, id: &UserID) -> Result<Vec<Case>, CaseError> {
    let cases = query
        .case_repo()
        .list(id)
        .await
        .map_err(|e| CaseError::Unknown(e.to_string()))?;

    Ok(cases)
}
