use crate::domain::cases::entity::CaseID;
use crate::domain::uow::case::CaseUoW;
use crate::domain::uow::common::UnitOfWorkFactory;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute<F: UnitOfWorkFactory>(uow: &CaseUoW<F>, id: &CaseID) -> Result<(), Error> {
    uow.settle(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
