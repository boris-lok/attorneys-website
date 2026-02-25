use crate::domain::entities::UserID;
use crate::repositories::{IUserRepository, IUserRolesRepository};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use secrecy::{ExposeSecret, SecretBox};

#[derive(Debug)]
pub struct Request {
    pub username: String,
    pub password: SecretBox<String>,
    pub nickname: String,
    pub role_ids: Vec<i16>,
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    req: Request,
    user_repo: tokio::sync::Mutex<impl IUserRepository + Sync + Send>,
    user_role_repo: tokio::sync::Mutex<impl IUserRolesRepository + Sync + Send>,
) -> Result<UserID, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::new(
        Algorithm::Argon2d,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(req.password.expose_secret().as_str().as_bytes(), &salt)
    .unwrap()
    .to_string();

    let mut lock = user_repo.lock().await;
    let user_id = lock
        .create_user(
            req.username,
            SecretBox::new(Box::new(password_hash)),
            req.nickname,
        )
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    let mut lock = user_role_repo.lock().await;

    for role_id in req.role_ids {
        lock.insert_user_role(user_id.clone(), role_id)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
    }

    Ok(user_id)
}
