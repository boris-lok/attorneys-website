use crate::domain::cases::entity::{CaseID, CreateCaseRequest};
use crate::domain::cases::error::CaseError;
use crate::domain::uow::case::CaseUoW;
use crate::domain::uow::common::UnitOfWorkFactory;

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &CaseUoW<F>,
    req: CreateCaseRequest,
) -> Result<CaseID, CaseError> {
    let id = uow.create(req).await?;

    Ok(id)
}
