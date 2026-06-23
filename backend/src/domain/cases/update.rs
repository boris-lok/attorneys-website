use crate::domain::cases::entity::UpdateCaseRequest;
use crate::domain::cases::error::CaseError;
use crate::domain::uow::case::CaseUoW;
use crate::domain::uow::common::UnitOfWorkFactory;

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &CaseUoW<F>,
    req: UpdateCaseRequest,
) -> Result<(), CaseError> {
    uow.update(req).await
}
