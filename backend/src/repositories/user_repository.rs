use crate::domain::entities::UserID;
use crate::repositories::Connection;
use anyhow::anyhow;
use secrecy::{ExposeSecret, SecretBox};
use sqlx::{Acquire, PgConnection, Row};
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IUserRepository {
    async fn get_credentials(
        &self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>>;

    async fn change_password(&self, id: UserID, password: SecretBox<String>) -> anyhow::Result<()>;

    async fn create_user(
        &self,
        username: String,
        password: SecretBox<String>,
    ) -> anyhow::Result<UserID>;
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

    async fn create_user(
        &self,
        username: String,
        password: SecretBox<String>,
    ) -> anyhow::Result<UserID> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.credentials.lock().await;
        let uuid = Uuid::new_v4();
        let id = UserID::from(uuid);
        lock.insert(id.clone(), (username, password));
        Ok(id)
    }
}

#[derive(Debug)]
pub struct SqlxUserRepository<'tx> {
    conn: Connection<'tx>,
}

impl<'tx> SqlxUserRepository<'tx> {
    pub fn new(conn: Connection<'tx>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl IUserRepository for SqlxUserRepository<'_> {
    async fn get_credentials(
        &self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                Ok(get_credentials(conn, username).await?)
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                Ok(get_credentials(conn, username).await?)
            }
        }
    }

    async fn change_password(&self, id: UserID, password: SecretBox<String>) -> anyhow::Result<()> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                Ok(change_password(conn, id, password).await?)
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                Ok(change_password(conn, id, password).await?)
            }
        }
    }

    async fn create_user(
        &self,
        username: String,
        password: SecretBox<String>,
    ) -> anyhow::Result<UserID> {
        match &self.conn {
            Connection::Pool(pool) => {
                let mut conn = pool.acquire().await?;
                let conn = conn.as_mut();

                let id = create_user(conn, username, password).await?;
                Ok(UserID::from(id))
            }
            Connection::Transaction(tx) => {
                let conn_ptr = tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
                let mut lock = conn_ptr.lock().await;
                let conn = lock.acquire().await?;

                let id = create_user(conn, username, password).await?;
                Ok(UserID::from(id))
            }
        }
    }
}

async fn get_credentials(
    conn: &mut PgConnection,
    username: &str,
) -> anyhow::Result<Option<(UserID, SecretBox<String>)>> {
    let query = "select id, password_hash from \"users\" where username = $1";

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

async fn change_password(
    conn: &mut PgConnection,
    id: UserID,
    password: SecretBox<String>,
) -> anyhow::Result<()> {
    let query = "UPDATE \"users\" SET password_hash = $1 WHERE id = $2";

    sqlx::query(query)
        .bind(password.expose_secret().to_string().as_str())
        .bind(id.to_string().as_str())
        .execute(conn)
        .await?;

    Ok(())
}

async fn create_user(
    conn: &mut PgConnection,
    username: String,
    password: SecretBox<String>,
) -> anyhow::Result<Uuid> {
    let uuid = Uuid::new_v4();
    let id = sqlx::query_scalar::<_, Uuid>(
        "insert into \"users\" (id, username, password_hash) values ($1, $2, $3) returning id;",
    )
    .bind(uuid)
    .bind(username)
    .bind(password.expose_secret().to_string())
    .fetch_one(conn)
    .await?;

    Ok(id)
}
