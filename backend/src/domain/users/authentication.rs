use crate::domain::entities::UserID;
use crate::repositories::IUserRepository;
use anyhow::{anyhow, Context};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, SecretBox};
use tokio::sync::Mutex;

pub struct Credentials {
    pub username: String,
    pub password: SecretBox<String>,
}

pub enum Error {
    InvalidCredentials,
    Unknown(String),
}

pub async fn validate_credentials(
    credentials: Credentials,
    user_repo: Mutex<impl IUserRepository + Sync + Send>,
) -> Result<UserID, Error> {
    let mut id = None;
    let mut expected_password_hash = SecretBox::new(Box::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    ));

    if let Some((user_id, password_hash)) = user_repo
        .lock()
        .await
        .get_credentials(credentials.username.as_str())
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?
    {
        id = Some(user_id);
        expected_password_hash = password_hash;
    }

    tokio::task::spawn_blocking(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?
    .map_err(|e| Error::InvalidCredentials)?;

    id.ok_or_else(|| anyhow!("Unknown username"))
        .map_err(|e| Error::InvalidCredentials)
}

fn verify_password_hash(
    expected_password_hash: SecretBox<String>,
    password_candidate: SecretBox<String>,
) -> anyhow::Result<()> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryUserRepository;
    use argon2::password_hash::SaltString;
    use argon2::{Algorithm, Params, PasswordHasher, Version};

    async fn helper() -> (InMemoryUserRepository, UserID, String, String) {
        let salt = SaltString::generate(&mut rand::thread_rng());

        let user_id = uuid::Uuid::new_v4();
        let user_id = UserID::try_from(user_id.to_string()).unwrap();
        let password = "password".to_string();
        let password_hash = Argon2::new(
            Algorithm::Argon2d,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(password.as_str().as_bytes(), &salt)
        .unwrap()
        .to_string();
        let username = "username".to_string();
        let secret_password = SecretBox::new(Box::new(password_hash));

        let user_repo = InMemoryUserRepository::new();
        user_repo
            .add_credentials(user_id.clone(), username.clone(), secret_password)
            .await;

        (user_repo, user_id, username, password)
    }

    #[tokio::test]
    async fn it_should_be_valid_password_otherwise() {
        let (repo, user_id, username, password) = helper().await;
        let repo = Mutex::new(repo);

        let credentials = Credentials {
            username: username.clone(),
            password: SecretBox::new(Box::new(password)),
        };

        let res = validate_credentials(credentials, repo).await;

        match res {
            Ok(id) => {
                assert_eq!(id, user_id)
            }
            Err(_) => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_be_invalid_password_otherwise() {
        let (repo, user_id, username, password) = helper().await;
        let repo = Mutex::new(repo);

        let credentials = Credentials {
            username: username.clone(),
            password: SecretBox::new(Box::new("wrong-password".to_string())),
        };

        let res = validate_credentials(credentials, repo).await;

        match res {
            Err(Error::InvalidCredentials) => {}
            _ => unreachable!(),
        }
    }
}
