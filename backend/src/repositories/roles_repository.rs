use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Role {
    pub id: i16,
    pub name: String,
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

#[async_trait::async_trait]
pub trait IRolesRepository {
    async fn list_roles(&self) -> Vec<Role>;
}

#[derive(Debug)]
pub struct InMemoryRolesRepository {
    roles: Mutex<Vec<Role>>,
}

impl InMemoryRolesRepository {
    pub fn new() -> Self {
        Self {
            roles: Mutex::new(vec![]),
        }
    }
}

#[async_trait::async_trait]
impl IRolesRepository for InMemoryRolesRepository {
    async fn list_roles(&self) -> Vec<Role> {
        let lock = self.roles.lock().await;

        (*lock).clone()
    }
}

pub struct SqlxRolesRepository<'tx> {
    conn: Mutex<&'tx mut sqlx::PgConnection>,
}

impl<'tx> SqlxRolesRepository<'tx> {
    pub fn new(conn: Mutex<&'tx mut sqlx::PgConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl IRolesRepository for SqlxRolesRepository<'_> {
    async fn list_roles(&self) -> Vec<Role> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = "select id, name from roles";

        let res = sqlx::query_as::<_, RoleFromSQLx>(query)
            .fetch_all(conn)
            .await;

        match res {
            Ok(res) => res.into_iter().map(Role::from).collect(),
            Err(_) => vec![],
        }
    }
}
