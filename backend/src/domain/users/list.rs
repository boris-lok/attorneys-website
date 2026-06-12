use crate::domain::uow::common::Query;
use crate::domain::users::entity::User;
use crate::domain::users::repository::UserReadRepository;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(query: &impl Query) -> Result<Vec<User>, Error> {
    query
        .user_repo()
        .list()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))
}
