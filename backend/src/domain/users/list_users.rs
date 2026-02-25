use crate::repositories::{IUserRepository, User};
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    user_repo: Mutex<impl IUserRepository + Sync + Send>,
) -> Result<Vec<User>, Error> {
    let lock = user_repo.lock().await;

    lock.list_users()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))
}
