use crate::domain::session::store::SessionStore;
use crate::domain::uow::common::UnitOfWorkFactory;
use crate::domain::uow::user::UserUoW;
use crate::domain::users::entity::UserID;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use secrecy::{ExposeSecret, SecretBox};
use std::sync::Arc;

pub struct Request {
    pub user_id: UserID,
    pub new_password: SecretBox<String>,
}

pub enum Error {
    Unknown(String),
}

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &UserUoW<F>,
    session: Arc<dyn SessionStore + Send + Sync>,
    req: Request,
) -> Result<(), Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::new(
        Algorithm::Argon2d,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(req.new_password.expose_secret().as_str().as_bytes(), &salt)
    .unwrap()
    .to_string();

    uow.change_password(&req.user_id, SecretBox::new(Box::new(password_hash)))
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    session
        .clear_user_sessions(&req.user_id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
