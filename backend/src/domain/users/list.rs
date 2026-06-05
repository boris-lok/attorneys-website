use crate::domain::users::entity::User;
use crate::domain::users::repository::UserRepository;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(repo: &mut impl UserRepository) -> Result<Vec<User>, Error> {
    repo.list().await.map_err(|e| Error::Unknown(e.to_string()))
}
