use crate::domain::entities::UserID;
use anyhow::anyhow;
use secrecy::{ExposeSecret, SecretBox};
use std::collections::HashMap;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IUserRepository {
    async fn get_credentials(
        &self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>>;

    async fn change_password(&self, id: UserID, password: SecretBox<String>) -> anyhow::Result<()>;
}

pub struct InMemoryUserRepository {
    error: bool,
    credentials: Mutex<HashMap<UserID, (String, SecretBox<String>)>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            credentials: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }

    #[cfg(test)]
    pub async fn add_credentials(&self, id: UserID, username: String, password: SecretBox<String>) {
        let mut lock = self.credentials.lock().await;
        lock.insert(id, (username, password));
    }
}

#[async_trait::async_trait]
impl IUserRepository for InMemoryUserRepository {
    async fn get_credentials(
        &self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let lock = self.credentials.lock().await;

        Ok(lock.iter().find_map(|(id, (name, password))| {
            if name == username {
                let pwd = password.expose_secret().to_string();
                let pwd = SecretBox::new(Box::new(pwd));
                Some((id.clone(), pwd))
            } else {
                None
            }
        }))
    }

    async fn change_password(&self, id: UserID, password: SecretBox<String>) -> anyhow::Result<()> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.credentials.lock().await;
        let entry = lock.entry(id);
        entry.and_modify(|(_, pwd)| *pwd = password);

        Ok(())
    }
}
