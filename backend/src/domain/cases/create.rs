use crate::domain::cases::entity::{CaseID, CreateCaseRequest};
use crate::domain::services::case::CaseUoW;
use crate::domain::uow::common::UnitOfWorkFactory;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &CaseUoW<F>,
    req: CreateCaseRequest,
) -> Result<CaseID, Error> {
    let id = uow
        .create(req)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(id)
}
