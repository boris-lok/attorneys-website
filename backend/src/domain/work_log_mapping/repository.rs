use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::entity::WorkLogMappingStatus;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait WorkLogMappingRepository {
    async fn create(&mut self, id: &Uuid, user_ids: Vec<UserID>) -> anyhow::Result<()>;

    async fn update_status(
        &mut self,
        id: &Uuid,
        user_id: &UserID,
        status: WorkLogMappingStatus,
    ) -> anyhow::Result<()>;
}
