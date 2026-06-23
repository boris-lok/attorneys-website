use crate::domain::cases::entity::CaseID;
use crate::domain::users::entity::UserID;
use crate::domain::work_logs::entity::{
    CreateWorkLogRequest, SimpleWorkLog, UpdateWorkLogRequest, WorkLog, WorkLogFilters,
};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait WorkLogsWriteRepository {
    async fn create(&mut self, req: CreateWorkLogRequest) -> anyhow::Result<()>;
    async fn delete(&mut self, id: &Uuid) -> anyhow::Result<()>;
    async fn update(&mut self, req: UpdateWorkLogRequest) -> anyhow::Result<()>;
    async fn settle(&mut self, case_id: &CaseID) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait WorkLogsReadRepository {
    async fn list(
        &mut self,
        case_id: &CaseID,
        filters: &WorkLogFilters,
    ) -> anyhow::Result<Vec<WorkLog>>;
    async fn is_creator(&mut self, id: &Uuid, user_id: &UserID) -> anyhow::Result<bool>;
    async fn is_collaborator_work_log(
        &mut self,
        id: &Uuid,
        user_id: &UserID,
    ) -> anyhow::Result<bool>;
    async fn is_work_log_exist(&mut self, id: &Uuid) -> anyhow::Result<bool>;
    async fn retrieve(&mut self, id: &Uuid) -> anyhow::Result<Option<SimpleWorkLog>>;
}

pub trait WorkLogsRepository: WorkLogsWriteRepository + WorkLogsReadRepository {}
