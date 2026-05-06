use crate::domain::entities::UserID;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct CaseID(Uuid);

impl From<CaseID> for String {
    fn from(id: CaseID) -> String {
        id.0.to_string()
    }
}

impl From<CaseID> for Uuid {
    fn from(value: CaseID) -> Self {
        value.0
    }
}

impl From<Uuid> for CaseID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for CaseID {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(&value).map_err(|_| String::from("invalid id"))?;
        Ok(CaseID(uuid))
    }
}

#[derive(Debug, Serialize)]
pub struct Case {
    pub id: CaseID,
    pub name: String,
    pub used_minutes: i32,
    pub estimated_minutes: i32,
    pub pending_logs: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
struct CaseFromSQLx {
    pub id: Uuid,
    pub name: String,
    pub used_minutes: i32,
    pub estimated_minutes: i32,
    pub pending_logs: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

impl From<CaseFromSQLx> for Case {
    fn from(value: CaseFromSQLx) -> Self {
        Self {
            id: CaseID(value.id),
            name: value.name,
            used_minutes: value.used_minutes,
            estimated_minutes: value.estimated_minutes,
            created_at: value.created_at,
            started_at: value.started_at,
            ended_at: value.ended_at,
            pending_logs: value.pending_logs,
        }
    }
}

#[async_trait::async_trait]
pub trait ICaseRepository {
    async fn create(
        &mut self,
        name: &str,
        estimated_minutes: i32,
        started_at: chrono::DateTime<chrono::Utc>,
        ended_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<CaseID>;
    async fn update(
        &mut self,
        id: CaseID,
        name: Option<String>,
        estimated_minutes: Option<i32>,
    ) -> anyhow::Result<()>;
    async fn list(&self, user_id: &UserID) -> anyhow::Result<Vec<Case>>;
    async fn delete(&mut self, id: CaseID) -> anyhow::Result<()>;
}

pub struct SQLxCaseRepository<'tx> {
    conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>,
}

impl<'tx> SQLxCaseRepository<'tx> {
    pub fn new(conn: Arc<Mutex<&'tx mut sqlx::PgConnection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl ICaseRepository for SQLxCaseRepository<'_> {
    async fn create(
        &mut self,
        name: &str,
        estimated_minutes: i32,
        started_at: chrono::DateTime<chrono::Utc>,
        ended_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<CaseID> {
        let id = Uuid::new_v4();

        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"insert into cases (id, name, estimated_minutes, started_at, ended_at) values ($1, $2, $3, $4, $5)";

        sqlx::query(query)
            .bind(id)
            .bind(name)
            .bind(estimated_minutes)
            .bind(started_at)
            .bind(ended_at)
            .execute(conn)
            .await?;

        Ok(CaseID(id))
    }

    async fn update(
        &mut self,
        id: CaseID,
        name: Option<String>,
        estimated_minutes: Option<i32>,
    ) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
    UPDATE cases
    SET estimated_minutes = COALESCE($1, estimated_minutes),
        name = COALESCE($2, name)
    WHERE id = $3
";

        sqlx::query(query)
            .bind(estimated_minutes) // Option<i32> or similar
            .bind(name) // Option<String>
            .bind(Uuid::from(id))
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn list(&self, user_id: &UserID) -> anyhow::Result<Vec<Case>> {
        let user_id = Uuid::from(user_id);

        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"
SELECT
    c.id,
    c.name,
    c.estimated_minutes,
    c.created_at,
    c.started_at,
    c.ended_at,

    COALESCE(
        SUM(
            wl.duration_minutes * (1 + COALESCE(cnt.approved_cnt, 0))
        ),
        0
    )::INT4 AS used_minutes,

    COALESCE(
        SUM(cnt.pending_cnt),
        0
    )::INT4 AS pending_logs

FROM cases c

LEFT JOIN work_logs wl
    ON c.id = wl.case_id
    AND wl.deleted_at IS NULL

LEFT JOIN (
    SELECT
        parent_id,
        COUNT(*) FILTER (WHERE status = 'approved') AS approved_cnt,
        COUNT(*) FILTER (WHERE status = 'pending' and user_id = $1) AS pending_cnt
    FROM work_logs_mapping
    GROUP BY parent_id
) cnt ON cnt.parent_id = wl.id

WHERE
    c.deleted_at IS NULL

GROUP BY
    c.id;
            ";

        let rows = sqlx::query_as::<_, CaseFromSQLx>(query)
            .bind(user_id)
            .fetch_all(conn)
            .await?;

        Ok(rows.into_iter().map(Case::from).collect())
    }

    async fn delete(&mut self, id: CaseID) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().await;
        let conn = &mut **conn;

        let query = r"update cases set deleted_at = now() where id = $1";

        sqlx::query(query)
            .bind(Uuid::from(id))
            .execute(conn)
            .await?;

        Ok(())
    }
}
