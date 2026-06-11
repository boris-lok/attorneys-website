use crate::domain::uow::common::UnitOfWorkFactory;
use crate::domain::uow::user::UserUoW;
use crate::domain::users::entity::UserID;
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

pub async fn execute<F: UnitOfWorkFactory>(
    service: &UserUoW<F>,
    req: Request,
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

    let id = service
        .create(
            req.username,
            SecretBox::new(Box::new(password_hash)),
            req.nickname,
            req.role_ids,
        )
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(id)
}
