use crate::domain::role::entity::Role;
use crate::domain::role::repository::RoleRepository;
use crate::infrastructure::db::connection::{PostgresRepo, RoleRepo};

const LIST_ROLES: &str = r"
  SELECT id, name FROM roles;
";

type PostgresRoleRepo<'tx> = PostgresRepo<'tx, RoleRepo>;

#[async_trait::async_trait]
impl<'tx> RoleRepository for PostgresRoleRepo<'tx> {
    async fn list(&mut self) -> Vec<Role> {
        let conn = self.conn().await;
        if conn.is_err() {
            return vec![];
        }
        let rows = sqlx::query_as::<_, RoleFromSQLx>(LIST_ROLES)
            .fetch_all(conn.unwrap())
            .await;

        match rows {
            Ok(rows) => rows.into_iter().map(Role::from).collect(),
            Err(_) => vec![],
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct RoleFromSQLx {
    id: i16,
    name: String,
}

impl From<RoleFromSQLx> for Role {
    fn from(value: RoleFromSQLx) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
