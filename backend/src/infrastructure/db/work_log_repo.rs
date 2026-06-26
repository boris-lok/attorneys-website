use crate::domain::cases::entity::CaseID;
use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::entity::WorkLogMappingStatus;
use crate::domain::work_logs::entity::{
    Collaborator, CreateWorkLogRequest, SimpleUser, SimpleWorkLog, UpdateWorkLogRequest, WorkLog,
    WorkLogFilters,
};
use crate::domain::work_logs::repository::{
    WorkLogsReadRepository, WorkLogsRepository, WorkLogsWriteRepository,
};
use crate::infrastructure::db::connection::{PostgresRepo, WorkLogRepo};
use crate::infrastructure::db::work_log_mapping_repo::PostgresWorkLogStatus;
use chrono::{DateTime, Utc};
use sqlx::Row;
use std::collections::HashMap;
use uuid::Uuid;

const CREATE_WORK_LOG_QUERY: &str = r"
  INSERT INTO work_logs
    (id, user_id, case_id, started_at, ended_at, description, is_collaborative)
  VALUES
    ($1, $2, $3, $4, $5, $6, $7);
";

const DELETE_WORK_LOG_QUERY: &str = r"
  UPDATE work_logs SET deleted_at = now() WHERE id = $1;
";

const UPDATE_WORK_LOG_QUERY: &str = r"
  UPDATE work_logs SET
    started_at = coalesce($1, started_at),
    ended_at = coalesce($2, ended_at),
    description = coalesce($3, description)
  WHERE id = $4;
";

const IS_CREATOR_QUERY: &str = r"
  SELECT EXISTS(SELECT 1 FROM work_logs WHERE user_id = $1 AND id = $2)
";

const GET_WORK_LOG_COLLABORATIVE_FLAG_QUERY: &str = r"
  SELECT is_collaborative FROM work_logs WHERE id = $1 and user_id = $2;
";

const IS_WORK_LOG_EXIST_QUERY: &str = r"
  SELECT EXISTS(SELECT 1 FROM work_logs WHERE id = $1)
";

const LIST_WORK_LOGS_QUERY: &str = r"
  select
    wl.id,
    wl.case_id,
    wl.started_at,
    wl.ended_at,
    wl.user_id,
    u.nickname as username,
    wl.duration_minutes as duration,
    wl.description,
    wl.is_collaborative,
    wlm.status,
    wlm.parent_id,
    wlm.user_id as collaborator_user_id,
    cu.nickname as collaborator_name,
    c.closed
  from work_logs wl
  join users u on u.id = wl.user_id
  join cases c on c.id = wl.case_id
  left join work_logs_mapping wlm on wlm.parent_id = wl.id
  left join users cu on wlm.user_id = cu.id
  where
    wl.case_id = $1
    and wl.deleted_at is null
    and ($2::timestamptz is null or wl.started_at >= $2)
    and ($3::timestamptz is null or wl.ended_at <= $3)
    and (
      $4::boolean is null
      or ($4::boolean = true and wl.settled_at is not null)
      or ($4::boolean = false and wl.settled_at is null)
    )
";

const SETTLE_WORK_LOGS_QUERY: &str = r"
  update work_logs set settled_at = now() where case_id = $1 and settled_at is null
";

const RETRIEVE_WORK_LOG_QUERY: &str = r"
  select id, case_id, is_collaborative from work_logs where id = $1;
";

pub type PostgresWorkLogRepo<'tx> = PostgresRepo<'tx, WorkLogRepo>;

#[async_trait::async_trait]
impl<'tx> WorkLogsWriteRepository for PostgresWorkLogRepo<'tx> {
    async fn create(&mut self, req: CreateWorkLogRequest) -> anyhow::Result<()> {
        sqlx::query(CREATE_WORK_LOG_QUERY)
            .bind(req.id)
            .bind(req.user_id)
            .bind(Uuid::from(&req.case_id))
            .bind(req.started_at)
            .bind(req.ended_at)
            .bind(req.description)
            .bind(req.is_collaborative)
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn delete(&mut self, id: &Uuid) -> anyhow::Result<()> {
        sqlx::query(DELETE_WORK_LOG_QUERY)
            .bind(id)
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn update(&mut self, req: UpdateWorkLogRequest) -> anyhow::Result<()> {
        sqlx::query(UPDATE_WORK_LOG_QUERY)
            .bind(req.started_at)
            .bind(req.ended_at)
            .bind(req.description)
            .bind(req.id)
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn settle(&mut self, case_id: &CaseID) -> anyhow::Result<()> {
        sqlx::query(SETTLE_WORK_LOGS_QUERY)
            .bind(Uuid::from(case_id))
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'tx> WorkLogsReadRepository for PostgresWorkLogRepo<'tx> {
    async fn list(
        &mut self,
        case_id: &CaseID,
        filters: &WorkLogFilters,
    ) -> anyhow::Result<Vec<WorkLog>> {
        let rows = sqlx::query_as::<_, WorkLogFromSQLx>(LIST_WORK_LOGS_QUERY)
            .bind(Uuid::from(case_id))
            .bind(filters.started_at)
            .bind(filters.ended_at)
            .bind(filters.settled)
            .fetch_all(self.conn().await?)
            .await?;

        Ok(parse_work_log_from_sqlx(rows))
    }

    async fn is_creator(&mut self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool> {
        let row = sqlx::query(IS_CREATOR_QUERY)
            .bind(Uuid::from(user_id))
            .bind(id)
            .fetch_one(self.conn().await?)
            .await?;

        Ok(!row.is_empty())
    }

    async fn is_collaborator_work_log(
        &mut self,
        id: &Uuid,
        user_id: &UserID,
    ) -> anyhow::Result<bool> {
        let res: Option<bool> = sqlx::query_scalar(GET_WORK_LOG_COLLABORATIVE_FLAG_QUERY)
            .bind(id)
            .bind(Uuid::from(user_id))
            .fetch_optional(self.conn().await?)
            .await?;

        res.ok_or_else(|| anyhow::anyhow!("work log not found: {}", id))
    }

    async fn is_work_log_exist(&mut self, id: &Uuid) -> anyhow::Result<bool> {
        let row = sqlx::query(IS_WORK_LOG_EXIST_QUERY)
            .bind(id)
            .fetch_one(self.conn().await?)
            .await?;

        Ok(!row.is_empty())
    }

    async fn retrieve(&mut self, id: &Uuid) -> anyhow::Result<Option<SimpleWorkLog>> {
        let res = sqlx::query_as::<_, SimpleWorkLogFromSQLx>(RETRIEVE_WORK_LOG_QUERY)
            .bind(id)
            .fetch_optional(self.conn().await?)
            .await?;

        Ok(res.map(|r| r.into()))
    }
}

impl<'tx> WorkLogsRepository for PostgresWorkLogRepo<'tx> {}

#[derive(Debug, sqlx::FromRow)]
pub struct SimpleWorkLogFromSQLx {
    id: Uuid,
    case_id: Uuid,
    is_collaborative: bool,
}

impl From<SimpleWorkLogFromSQLx> for SimpleWorkLog {
    fn from(value: SimpleWorkLogFromSQLx) -> Self {
        Self {
            id: value.id,
            case_id: CaseID::from(value.case_id),
            is_collaborative: value.is_collaborative,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct WorkLogFromSQLx {
    id: Uuid,
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
    user_id: Uuid,
    username: String,
    duration: i32,
    description: String,
    is_collaborative: bool,
    status: Option<PostgresWorkLogStatus>,
    parent_id: Option<Uuid>,
    collaborator_user_id: Option<Uuid>,
    collaborator_name: Option<String>,
    closed: bool,
}

fn parse_work_log_from_sqlx(rows: Vec<WorkLogFromSQLx>) -> Vec<WorkLog> {
    let mut res: HashMap<_, _> = HashMap::new();
    let mut collaborators: Vec<(Uuid, Collaborator)> = Vec::new();

    for row in rows {
        // Ensure parent WorkLog exists
        res.entry(row.id).or_insert_with(|| WorkLog {
            id: row.id,
            started_at: row.started_at,
            ended_at: row.ended_at,
            user: SimpleUser {
                id: row.user_id,
                name: row.username,
            },
            duration: row.duration,
            description: row.description,
            is_collaborative: row.is_collaborative,
            collaborators: vec![],
            closed: row.closed,
        });

        // Attach collaborator if present
        if let Some(parent_id) = row.parent_id {
            collaborators.push((
                parent_id,
                Collaborator {
                    parent_id,
                    user_id: row.collaborator_user_id.unwrap(),
                    name: row.collaborator_name.unwrap(),
                    status: row.status.map(WorkLogMappingStatus::from).unwrap().into(),
                },
            ));
        }
    }

    // Second pass: safe to attach now that all parents exist
    for (parent_id, collaborator) in collaborators {
        if let Some(parent) = res.get_mut(&parent_id) {
            parent.collaborators.push(collaborator);
        }
    }

    let mut sorted: Vec<_> = res.into_values().collect();
    sorted.sort_by_key(|wl| wl.started_at);

    sorted
}
