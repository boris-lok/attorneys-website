use crate::domain::cases::entity::CaseID;
use crate::domain::cases::error::CaseError;
use crate::domain::uow::case::CaseUoW;
use crate::domain::uow::common::UnitOfWorkFactory;

pub async fn execute<F: UnitOfWorkFactory>(uow: &CaseUoW<F>, id: &CaseID) -> Result<(), CaseError> {
    uow.settle(id).await
}
