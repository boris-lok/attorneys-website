use crate::domain::entities::UserID;
use crate::repositories::IUserRepository;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use secrecy::{ExposeSecret, SecretBox};

pub(crate) struct Request {
    pub(crate) user_id: UserID,
    pub(crate) new_password: SecretBox<String>,
}

pub(crate) enum Error {
    Unknown(String),
}

pub async fn execute(
    req: Request,
    user_repo: tokio::sync::Mutex<impl IUserRepository + Sync + Send>,
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

    let lock = user_repo.lock().await;
    lock.change_password(req.user_id, SecretBox::new(Box::new(password_hash)))
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}
