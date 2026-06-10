use crate::domain::users::entity::{User, UserID};
use crate::domain::users::repository::UserRepository;
use crate::infrastructure::db::connection::{PostgresRepo, UserRepo};
use secrecy::{ExposeSecret, SecretBox};
use uuid::Uuid;

const CREATE_USER_QUERY: &str = r"
  INSERT INTO users (username, password_hash, nickname) VALUES ($1, $2, $3) RETURNING id;
";

const LIST_USERS_QUERY: &str = r"
  SELECT
    u.id,
    u.username,
    u.nickname,
    array_agg(r.name) as roles
  FROM users u
  LEFT JOIN user_roles ur ON u.id = ur.user_id
  LEFT JOIN roles r ON ur.role_id = r.id
  GROUP BY u.id, u.username, u.nickname
";

const DELETE_USER_QUERY: &str = r"
  UPDATE users SET deleted_at = NOW() WHERE id = $1;
";

const GET_USER_NICKNAME_QUERY: &str = r"
  SELECT nickname FROM users WHERE id = $1 and deleted_at is null;
";

const GET_CREDENTIALS_QUERY: &str = r"
  SELECT id, password_hash FROM users WHERE username = $1 and deleted_at is null;
";

const UPDATE_PASSWORD_QUERY: &str = r"
  UPDATE users SET password_hash = $1 WHERE id = $2;
";

pub type PostgresUserRepo<'tx> = PostgresRepo<'tx, UserRepo>;

#[async_trait::async_trait]
impl<'tx> UserRepository for PostgresUserRepo<'tx> {
    async fn create(
        &mut self,
        username: String,
        password_hash: SecretBox<String>,
        nickname: String,
    ) -> anyhow::Result<UserID> {
        let id = sqlx::query_scalar::<_, Uuid>(CREATE_USER_QUERY)
            .bind(username)
            .bind(password_hash.expose_secret().to_string())
            .bind(nickname)
            .fetch_one(self.conn().await?)
            .await
            .map(UserID::from)?;

        Ok(id)
    }

    async fn list(&mut self) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query_as::<_, UserFromSQLx>(LIST_USERS_QUERY)
            .fetch_all(self.conn().await?)
            .await
            .map(|rows| rows.into_iter().map(User::from).collect::<Vec<_>>())?;

        Ok(rows)
    }

    async fn delete(&mut self, id: &UserID) -> anyhow::Result<()> {
        sqlx::query(DELETE_USER_QUERY)
            .bind(Uuid::from(id))
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn get_user_nickname(&mut self, id: &UserID) -> anyhow::Result<String> {
        let res = sqlx::query_scalar::<_, String>(GET_USER_NICKNAME_QUERY)
            .bind(Uuid::from(id))
            .fetch_optional(self.conn().await?)
            .await?;

        Ok(res.ok_or_else(|| anyhow::anyhow!("User not found"))?)
    }

    async fn get_credentials(
        &mut self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>> {
        let res = sqlx::query_as::<_, (Uuid, String)>(GET_CREDENTIALS_QUERY)
            .bind(username)
            .fetch_optional(self.conn().await?)
            .await
            .map(|e| {
                e.map(|(id, password_hash)| {
                    (UserID::from(id), SecretBox::new(Box::new(password_hash)))
                })
            })?;

        Ok(res)
    }

    async fn change_password(
        &mut self,
        id: &UserID,
        password_hash: SecretBox<String>,
    ) -> anyhow::Result<()> {
        sqlx::query(UPDATE_PASSWORD_QUERY)
            .bind(password_hash.expose_secret().to_string())
            .bind(Uuid::from(id))
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }
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
