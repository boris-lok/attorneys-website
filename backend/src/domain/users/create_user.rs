use crate::domain::entities::UserID;
use crate::repositories::IUserRepository;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use secrecy::{ExposeSecret, SecretBox};

#[derive(Debug)]
pub struct Request {
    pub username: String,
    pub password: SecretBox<String>,
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute(
    req: Request,
    user_repo: tokio::sync::Mutex<impl IUserRepository + Sync + Send>,
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

    let lock = user_repo.lock().await;
    lock.create_user(req.username, SecretBox::new(Box::new(password_hash)))
        .await
        .map_err(|e| Error::Unknown(e.to_string()))
}
