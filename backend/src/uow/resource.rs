use crate::domain::entities::{ContentID, Language, ResourceID};
use crate::repositories::IResourceRepository;
use crate::repositories::{IAvatarRepository, InMemoryAvatarRepository, InMemoryContentRepository};
use crate::repositories::{IContentRepository, InMemoryResourceRepository};
use serde::de::DeserializeOwned;

/** Define a unit of work to organize all related repositories.
*
* - resource repository
* - content repository
* - avatar repository
*/
#[async_trait::async_trait]
pub trait IResourceUnitOfWork {
    /** Resource repository stores the data by different types (e.g. members, services, home, etc.) */
    fn resource_repository(&mut self) -> &mut impl IResourceRepository;

    /** Content repository stores multiple language data */
    fn content_repository(&mut self) -> &mut impl IContentRepository;

    /** Avatar repository stores all avatars associated with the members. */
    fn avatar_repository(&mut self) -> &mut impl IAvatarRepository;

    /** Get a resource by ID and language */
    async fn get_resource<T>(&self, id: &ResourceID, lang: &Language) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned;

    /** Commit the transaction */
    async fn commit(mut self) -> anyhow::Result<()>;
    /** Rollback the transaction */
    async fn rollback(mut self) -> anyhow::Result<()>;
}

pub struct InMemoryResource {
    error: bool,
    resource_repository: Option<InMemoryResourceRepository>,
    content_repository: Option<InMemoryContentRepository>,
    avatar_repository: Option<InMemoryAvatarRepository>,
}

impl InMemoryResource {
    pub fn new() -> Self {
        Self {
            error: false,
            resource_repository: None,
            content_repository: None,
            avatar_repository: None,
        }
    }

    pub fn with_error(mut self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

#[async_trait::async_trait]
impl IResourceUnitOfWork for InMemoryResource {
    fn resource_repository(&mut self) -> &mut impl IResourceRepository {
        if self.resource_repository.is_none() {
            let resource_repo = if self.error {
                InMemoryResourceRepository::new().with_error()
            } else {
                InMemoryResourceRepository::new()
            };
            self.resource_repository = Some(resource_repo);
        }
        self.resource_repository.as_mut().unwrap()
    }

    fn content_repository(&mut self) -> &mut impl IContentRepository {
        if self.content_repository.is_none() {
            let content_repo = if self.error {
                InMemoryContentRepository::new().with_error()
            } else {
                InMemoryContentRepository::new()
            };
            self.content_repository = Some(content_repo);
        }
        self.content_repository.as_mut().unwrap()
    }

    fn avatar_repository(&mut self) -> &mut impl IAvatarRepository {
        if self.avatar_repository.is_none() {
            let avatar_repo = if self.error {
                InMemoryAvatarRepository::new().with_error()
            } else {
                InMemoryAvatarRepository::new()
            };
            self.avatar_repository = Some(avatar_repo);
        }
        self.avatar_repository.as_mut().unwrap()
    }

    async fn get_resource<T>(&self, id: &ResourceID, lang: &Language) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let content_id = ContentID::from(id.clone());
        let data = self
            .content_repository
            .as_ref()
            .unwrap()
            .get(&content_id, &lang)
            .await;

        match data {
            Ok(Some(data)) => {
                let obj = serde_json::from_value::<T>(data.clone().to_json())?;
                Ok(Some(obj))
            }
            Err(e) => Err(e),
            Ok(None) => Ok(None),
        }
    }

    async fn commit(mut self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn rollback(mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
