use crate::domain::cases::entity::{
    Case, CaseID, CreateCaseRequest, SimpleCase, UpdateCaseRequest,
};
use crate::domain::cases::repository::{CaseReadRepository, CaseRepository, CaseWriteRepository};
use crate::domain::users::entity::UserID;
use crate::infrastructure::db::connection::{CaseRepo, PostgresRepo};
use uuid::Uuid;

const CREATE_CASE_QUERY: &str = r"
  INSERT INTO cases
    (id, name, estimated_minutes, started_at, ended_at, billing_cycle)
  VALUES
    ($1, $2, $3, $4, $5, $6)
";

const UPDATE_CASE_QUERY: &str = r"
  UPDATE cases SET
    estimated_minutes = COALESCE($2, estimated_minutes),
    name = COALESCE($3, name),
    billing_cycle = COALESCE($4, billing_cycle),
    started_at = COALESCE($5, started_at),
    ended_at = COALESCE($6, ended_at),
    closed = COALESCE($7, closed)
  WHERE id = $1
";

const DELETE_CASE_QUERY: &str = r"
  UPDATE cases SET
    deleted_at = now()
  WHERE id = $1
";

const SETTLE_CASE_QUERY: &str = r"
  UPDATE cases SET
    settled_at = now()
  WHERE id = $1
";

const LIST_CASES_QUERY: &str = r"
  SELECT
    c.id,
    c.name,
    c.estimated_minutes,
    c.billing_cycle,
    c.created_at,
    c.started_at,
    c.ended_at,
    c.settled_at,
    COALESCE(
        SUM(
            wl.duration_minutes * (1 + COALESCE(cnt.approved_cnt, 0))
        ),
        0
    )::INT4 AS used_minutes,
    COALESCE(
        SUM(cnt.pending_cnt),
        0
    )::INT4 AS pending_logs,
    c.closed
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
    c.id
";

const RETRIEVE_CASE_QUERY: &str = r"
  select id, closed from cases where id = $1;
";

pub type PostgresCaseRepo<'tx> = PostgresRepo<'tx, CaseRepo>;

#[async_trait::async_trait]
impl<'tx> CaseWriteRepository for PostgresCaseRepo<'tx> {
    async fn create(&mut self, req: CreateCaseRequest) -> anyhow::Result<CaseID> {
        sqlx::query(CREATE_CASE_QUERY)
            .bind(Uuid::from(&req.id))
            .bind(req.name)
            .bind(req.estimated_minutes)
            .bind(req.started_at)
            .bind(req.ended_at)
            .bind(req.billing_cycle)
            .execute(self.conn().await?)
            .await?;

        Ok(req.id)
    }

    async fn update(&mut self, req: UpdateCaseRequest) -> anyhow::Result<()> {
        sqlx::query(UPDATE_CASE_QUERY)
            .bind(Uuid::from(&req.id))
            .bind(req.estimated_minutes)
            .bind(req.name)
            .bind(req.billing_cycle)
            .bind(req.started_at)
            .bind(req.ended_at)
            .bind(req.closed)
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn delete(&mut self, case_id: &CaseID) -> anyhow::Result<()> {
        sqlx::query(DELETE_CASE_QUERY)
            .bind(Uuid::from(case_id))
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn settle(&mut self, case_id: &CaseID) -> anyhow::Result<()> {
        sqlx::query(SETTLE_CASE_QUERY)
            .bind(Uuid::from(case_id))
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'tx> CaseReadRepository for PostgresCaseRepo<'tx> {
    async fn list(&mut self, user_id: &UserID) -> anyhow::Result<Vec<Case>> {
        let res = sqlx::query_as::<_, CaseFromSQLx>(LIST_CASES_QUERY)
            .bind(Uuid::from(user_id))
            .fetch_all(self.conn().await?)
            .await?;

        let cases = res.into_iter().map(|r| r.into()).collect::<Vec<Case>>();
        Ok(cases)
    }

    async fn retrieve(&mut self, case_id: &CaseID) -> anyhow::Result<Option<SimpleCase>> {
        let res = sqlx::query_as::<_, SimpleCaseFromSQLx>(RETRIEVE_CASE_QUERY)
            .bind(Uuid::from(case_id))
            .fetch_optional(self.conn().await?)
            .await?;

        Ok(res.map(|r| r.into()))
    }
}

impl<'tx> CaseRepository for PostgresCaseRepo<'tx> {}

#[derive(Debug, sqlx::FromRow)]
struct CaseFromSQLx {
    pub id: Uuid,
    pub name: String,
    pub used_minutes: i32,
    pub estimated_minutes: i32,
    pub billing_cycle: i32,
    pub pending_logs: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
    pub settled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub closed: bool,
}

impl From<CaseFromSQLx> for Case {
    fn from(value: CaseFromSQLx) -> Self {
        Self {
            id: CaseID::from(value.id),
            name: value.name,
            used_minutes: value.used_minutes,
            estimated_minutes: value.estimated_minutes,
            billing_cycle: value.billing_cycle,
            created_at: value.created_at,
            started_at: value.started_at,
            ended_at: value.ended_at,
            pending_logs: value.pending_logs,
            settled_at: value.settled_at,
            closed: value.closed,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SimpleCaseFromSQLx {
    pub id: Uuid,
    pub closed: bool,
}

impl From<SimpleCaseFromSQLx> for SimpleCase {
    fn from(value: SimpleCaseFromSQLx) -> Self {
        Self {
            id: CaseID::from(value.id),
            closed: value.closed,
        }
    }
}
