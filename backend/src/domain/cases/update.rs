use crate::domain::cases::entity::UpdateCaseRequest;
use crate::domain::services::case::CaseUoW;
use crate::domain::uow::common::UnitOfWorkFactory;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &CaseUoW<F>,
    req: UpdateCaseRequest,
) -> Result<(), Error> {
    uow.update(req)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))
}
