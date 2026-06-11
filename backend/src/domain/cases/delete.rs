use crate::domain::cases::entity::CaseID;
use crate::domain::services::case::CaseUoW;
use crate::domain::uow::common::UnitOfWorkFactory;

pub enum Error {
    Unknown(String),
}

pub async fn execute<F: UnitOfWorkFactory>(uow: &CaseUoW<F>, id: &CaseID) -> Result<(), Error> {
    uow.delete(id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
