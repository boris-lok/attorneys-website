use crate::domain::cases::entity::{Case, CaseID, CreateCaseRequest, UpdateCaseRequest};
use crate::domain::users::entity::UserID;

#[async_trait::async_trait]
pub trait CaseWriteRepository {
    async fn create(&mut self, req: CreateCaseRequest) -> anyhow::Result<CaseID>;
    async fn update(&mut self, req: UpdateCaseRequest) -> anyhow::Result<()>;
    async fn delete(&mut self, case_id: &CaseID) -> anyhow::Result<()>;
    async fn settle(&mut self, case_id: &CaseID) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait CaseReadRepository {
    async fn list(&mut self, user_id: &UserID) -> anyhow::Result<Vec<Case>>;
}

pub trait CaseRepository: CaseReadRepository + CaseWriteRepository {}
