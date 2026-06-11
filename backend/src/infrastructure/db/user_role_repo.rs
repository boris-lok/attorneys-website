use crate::domain::users::entity::UserID;
use crate::domain::users::repository::{
    UserRoleReadRepository, UserRoleRepository, UserRoleWriteRepository,
};
use crate::infrastructure::db::connection::{PostgresRepo, UserRoleRepo};
use uuid::Uuid;

const GET_USER_ROLES_QUERY: &str = r"
  SELECT
    roles.name
  FROM
    user_roles, roles
  WHERE
    roles.id = user_roles.role_id
    and user_roles.user_id = $1;
";

const CREATE_USER_ROLE_QUERY: &str = r"
  INSERT INTO user_roles (user_id, role_id)
  VALUES ($1, $2);
";

type PostgresUserRoleRepo<'tx> = PostgresRepo<'tx, UserRoleRepo>;

#[async_trait::async_trait]
impl<'tx> UserRoleWriteRepository for PostgresUserRoleRepo<'tx> {
    async fn create(&mut self, user_id: &UserID, role: i16) -> anyhow::Result<()> {
        sqlx::query(CREATE_USER_ROLE_QUERY)
            .bind(Uuid::from(user_id))
            .bind(role)
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'tx> UserRoleReadRepository for PostgresUserRoleRepo<'tx> {
    async fn get_user_roles(&mut self, id: &UserID) -> anyhow::Result<Vec<String>> {
        let res = sqlx::query_scalar::<_, String>(GET_USER_ROLES_QUERY)
            .bind(Uuid::from(id))
            .fetch_all(self.conn().await?)
            .await?;

        Ok(res)
    }
}

impl UserRoleRepository for PostgresUserRoleRepo<'_> {}
