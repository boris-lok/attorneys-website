use crate::domain::entity::Pagination;
use crate::domain::resources::entity::{
    ContentData, ContentID, Language, ResourceID, ResourceType,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait::async_trait]
pub trait ContentWriteRepository {
    async fn create(
        &mut self,
        id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()>;
    async fn update(
        &mut self,
        id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait ResourceWriteRepository {
    async fn create(&mut self, id: &ResourceID, t: &ResourceType, seq: i32) -> anyhow::Result<()>;
    async fn delete(&mut self, id: &ResourceID) -> anyhow::Result<()>;
    async fn update_seq(&mut self, id: &ResourceID, seq: i32) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait ResourceReadRepository {
    async fn retrieve<T: DeserializeOwned + Serialize>(
        &mut self,
        id: &ResourceID,
        lang: &Language,
        kind: &ResourceType,
    ) -> anyhow::Result<Option<T>>;
    async fn list<T: DeserializeOwned + Serialize>(
        &mut self,
        lang: &Language,
        kind: &ResourceType,
        filter_str: &str,
        page: &Pagination,
    ) -> anyhow::Result<Vec<T>>;
    async fn count(
        &mut self,
        lang: &Language,
        kind: &ResourceType,
        filter_str: &str,
    ) -> anyhow::Result<i64>;
}

pub trait ResourceRepository: ResourceReadRepository + ResourceWriteRepository {}
