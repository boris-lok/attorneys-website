use crate::domain::cases::entity::CaseID;
use crate::domain::users::entity::UserID;
use crate::domain::work_logs::entity::{CreateWorkLogRequest, UpdateWorkLogRequest, WorkLog};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait WorkLogsRepository {
    async fn create(&mut self, req: CreateWorkLogRequest) -> anyhow::Result<()>;

    async fn delete(&mut self, id: &Uuid) -> anyhow::Result<()>;

    async fn list(
        &mut self,
        case_id: &CaseID,
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        ended_at: Option<chrono::DateTime<chrono::Utc>>,
        include_settled: bool,
    ) -> anyhow::Result<Vec<WorkLog>>;

    async fn update(&mut self, req: UpdateWorkLogRequest) -> anyhow::Result<()>;

    async fn is_creator(&mut self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool>;

    async fn is_collaborator_work_log(
        &mut self,
        id: &Uuid,
        user_id: &UserID,
    ) -> anyhow::Result<bool>;

    async fn is_work_log_exist(&mut self, id: &Uuid) -> anyhow::Result<bool>;
}
