use crate::domain::entities::UserID;
use anyhow::anyhow;
use secrecy::{ExposeSecret, SecretBox};
use sqlx::{Acquire, Postgres, Row, Transaction};
use std::collections::HashMap;
use std::sync::Weak;
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

#[cfg(test)]
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

#[derive(Debug)]
pub struct SqlxUserRepository<'tx> {
    tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
}

impl<'tx> SqlxUserRepository<'tx> {
    pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl<'tx> IUserRepository for SqlxUserRepository<'tx> {
    async fn get_credentials(
        &self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>> {
        let query = "select id, password_hash from \"users\" where username = $1";

        let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
        let mut lock = conn_ptr.lock().await;
        let conn = lock.acquire().await?;

        let res = sqlx::query(query)
            .bind(username)
            .fetch_optional(conn)
            .await
            .map(|e| match e {
                None => None,
                Some(row) => {
                    let id = row.get::<uuid::Uuid, usize>(0);
                    let id = UserID::from(id);
                    let password_hash = row.get::<String, usize>(1);

                    Some((id, SecretBox::new(Box::new(password_hash))))
                }
            })?;

        Ok(res)
    }

    async fn change_password(&self, id: UserID, password: SecretBox<String>) -> anyhow::Result<()> {
        todo!()
    }
}
