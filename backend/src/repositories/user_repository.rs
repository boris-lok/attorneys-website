use crate::domain::entities::UserID;
use secrecy::{ExposeSecret, SecretBox};
use sqlx::{PgConnection, Row};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: UserID,
    pub username: String,
    pub nickname: String,
    pub roles: Vec<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct UserFromSQLx {
    pub id: Uuid,
    pub username: String,
    pub nickname: String,
    pub roles: Vec<String>,
}

impl From<UserFromSQLx> for User {
    fn from(value: UserFromSQLx) -> Self {
        Self {
            id: UserID::from(value.id),
            username: value.username,
            nickname: value.nickname,
            roles: value.roles,
        }
    }
}

#[async_trait::async_trait]
pub trait IUserRepository {
    async fn get_credentials(
        &self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>>;
    async fn change_password(
        &mut self,
        id: UserID,
        password: SecretBox<String>,
    ) -> anyhow::Result<()>;
    async fn create_user(
        &mut self,
        username: String,
        password: SecretBox<String>,
        nickname: String,
    ) -> anyhow::Result<UserID>;

    async fn list_users(&self) -> anyhow::Result<Vec<User>>;
    async fn delete_user(&self, id: &UserID) -> anyhow::Result<()>;
    async fn get_user_roles(&self, id: &UserID) -> anyhow::Result<Vec<String>>;
    async fn get_user_nickname(&self, id: &UserID) -> anyhow::Result<String>;
}

#[cfg(test)]
use anyhow::anyhow;
use sqlx::postgres::PgRow;
#[cfg(test)]
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(test)]
pub struct InMemoryUserRepository {
    error: bool,
    credentials: Mutex<HashMap<UserID, (String, SecretBox<String>)>>,
}

#[cfg(test)]
impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl InMemoryUserRepository {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            error: false,
            credentials: Mutex::new(HashMap::new()),
        }
    }

    #[cfg(test)]
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
#[cfg(test)]
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

    async fn change_password(
        &mut self,
        id: UserID,
        password: SecretBox<String>,
    ) -> anyhow::Result<()> {
        if self.error {
            return Err(anyhow!("Internal Server Error"));
        }

        let mut lock = self.credentials.lock().await;
        let entry = lock.entry(id);
        entry.and_modify(|(_, pwd)| *pwd = password);

        Ok(())
    }

    async fn create_user(
        &mut self,
        username: String,
        password: SecretBox<String>,
        _: String,
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

    async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        todo!()
    }

    async fn delete_user(&self, id: &UserID) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_user_roles(&self, id: &UserID) -> anyhow::Result<Vec<String>> {
        todo!()
    }

    async fn get_user_nickname(&self, id: &UserID) -> anyhow::Result<String> {
        todo!()
    }
}

#[derive(Debug)]
pub struct SqlxUserRepository<'tx> {
    conn: Arc<Mutex<&'tx mut PgConnection>>,
}

impl<'tx> SqlxUserRepository<'tx> {
    pub fn new(conn: Arc<Mutex<&'tx mut PgConnection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl IUserRepository for SqlxUserRepository<'_> {
    async fn get_credentials(
        &self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query =
            "select id, password_hash from \"users\" where username = $1 and deleted_at is null;";

        let res = sqlx::query(query).bind(username).fetch_optional(conn).await;

        Ok(res.map(|e| match e {
            None => None,
            Some(row) => {
                let id = row.get::<uuid::Uuid, usize>(0);
                let id = UserID::from(id);
                let password_hash = row.get::<String, usize>(1);

                Some((id, SecretBox::new(Box::new(password_hash))))
            }
        })?)
    }

    async fn change_password(
        &mut self,
        id: UserID,
        password: SecretBox<String>,
    ) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = "UPDATE \"users\" SET password_hash = $1 WHERE id = $2";

        sqlx::query(query)
            .bind(password.expose_secret().to_string().as_str())
            .bind(id.to_string().as_str())
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn create_user(
        &mut self,
        username: String,
        password: SecretBox<String>,
        nickname: String,
    ) -> anyhow::Result<UserID> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let uuid = Uuid::new_v4();

        let id = sqlx::query_scalar::<_, Uuid>(
            "insert into \"users\" (id, username, nickname, password_hash) values ($1, $2, $3, $4) returning id;",
        )
            .bind(uuid)
            .bind(username)
            .bind(nickname)
            .bind(password.expose_secret().to_string())
            .fetch_one(conn)
            .await?;

        Ok(UserID::from(id))
    }

    async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        select u.id, u.username, u.nickname, array_agg(r.name) as roles 
        from users u
        left join user_roles ur on u.id = ur.user_id
        left join roles r on ur.role_id = r.id
        group by u.id
        ";

        let res = sqlx::query_as::<_, UserFromSQLx>(query)
            .fetch_all(conn)
            .await?;
        Ok(res.into_iter().map(Into::into).collect())
    }

    async fn delete_user(&self, id: &UserID) -> anyhow::Result<()> {
        let id = Uuid::from(id);

        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = "update \"users\" set deleted_at = NOW() where id = $1";

        sqlx::query(query).bind(id).execute(conn).await?;
        Ok(())
    }

    async fn get_user_roles(&self, id: &UserID) -> anyhow::Result<Vec<String>> {
        let id = Uuid::from(id);

        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
        select
          roles.name
        from
          user_roles, roles
        where
          roles.id = user_roles.role_id
          and user_roles.user_id = $1;
        ";

        let res = sqlx::query(query).bind(id).fetch_all(conn).await?;
        Ok(res.into_iter().map(|row: PgRow| row.get(0)).collect())
    }

    async fn get_user_nickname(&self, id: &UserID) -> anyhow::Result<String> {
        let id = Uuid::from(id);

        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = "select nickname from users where id = $1";
        let res = sqlx::query(query).bind(id).fetch_one(conn).await?;
        Ok(res.get(0))
    }
}
