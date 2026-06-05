use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::entity::WorkLogMappingStatus;
use crate::domain::work_log_mapping::repository::WorkLogMappingRepository;
use crate::infrastructure::db::connection::{PostgresRepo, WorkLogMappingRepo};
use sqlx::Postgres;
use uuid::Uuid;

const UPDATE_STATUS_QUERY: &str = r"
  UPDATE work_logs_mapping SET status = $3 WHERE parent_id = $1 AND user_id = $2;
";

pub type PostgresWorkLogMappingRepo<'tx> = PostgresRepo<'tx, WorkLogMappingRepo>;

#[async_trait::async_trait]
impl<'tx> WorkLogMappingRepository for PostgresWorkLogMappingRepo<'tx> {
    async fn create(&mut self, id: &Uuid, user_ids: Vec<UserID>) -> anyhow::Result<()> {
        // Guard against empty input
        if user_ids.is_empty() {
            return Ok(());
        }

        let mut qb = sqlx::QueryBuilder::<Postgres>::new(
            "insert into work_logs_mapping (parent_id, user_id, status)",
        );

        qb.push_values(user_ids, |mut b, user_id| {
            let user_id = Uuid::from(&user_id);
            b.push_bind(id)
                .push_bind(user_id)
                .push_bind(PostgresWorkLogStatus::Pending);
        });

        // Conflict handling — adjust to DO UPDATE if you need upsert behaviour
        qb.push(" ON CONFLICT (parent_id, user_id) DO NOTHING");

        qb.build().execute(self.get_conn()).await?;

        Ok(())
    }

    async fn update_status(
        &mut self,
        id: &Uuid,
        user_id: &UserID,
        status: WorkLogMappingStatus,
    ) -> anyhow::Result<()> {
        sqlx::query(UPDATE_STATUS_QUERY)
            .bind(id)
            .bind(Uuid::from(user_id))
            .bind(PostgresWorkLogStatus::from(status))
            .execute(self.get_conn())
            .await?;
        Ok(())
    }
}

#[derive(sqlx::Type, Clone, Debug)]
#[sqlx(type_name = "work_log_status", rename_all = "lowercase")]
pub enum PostgresWorkLogStatus {
    Pending,
    Rejected,
    Approved,
}

impl From<WorkLogMappingStatus> for PostgresWorkLogStatus {
    fn from(value: WorkLogMappingStatus) -> Self {
        match value {
            WorkLogMappingStatus::Pending => PostgresWorkLogStatus::Pending,
            WorkLogMappingStatus::Rejected => PostgresWorkLogStatus::Rejected,
            WorkLogMappingStatus::Approved => PostgresWorkLogStatus::Approved,
        }
    }
}

impl From<PostgresWorkLogStatus> for WorkLogMappingStatus {
    fn from(value: PostgresWorkLogStatus) -> Self {
        match value {
            PostgresWorkLogStatus::Pending => WorkLogMappingStatus::Pending,
            PostgresWorkLogStatus::Rejected => WorkLogMappingStatus::Rejected,
            PostgresWorkLogStatus::Approved => WorkLogMappingStatus::Approved,
        }
    }
}
