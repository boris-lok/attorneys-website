use crate::domain::entities::{ResourceID, ResourceType};

#[async_trait::async_trait]
pub trait IResourceRepository {
    async fn insert(
        &self,
        id: ResourceID,
        resource_type: ResourceType,
    ) -> anyhow::Result<ResourceID>;
}
