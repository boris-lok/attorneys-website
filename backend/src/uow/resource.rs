use crate::domain::entities::{
    ArticleData, ArticleEntity, ArticleEntityFromSQLx, ContactData, ContactEntity,
    ContactEntityFromSQLx, ContentID, HomeData, HomeEntity, HomeEntityFromSQLx, Language,
    MemberData, MemberEntity, MemberEntityFromSQLx, Pagination, ResourceID, ResourceType,
    ServiceData, ServiceEntity, ServiceEntityFromSQLx, SimpleMemberEntity,
    SimpleMemberEntityFromSQLx,
};
use crate::domain::member::entities::AvatarData;
use crate::repositories::{
    IAvatarRepository, InMemoryAvatarRepository, InMemoryContentRepository, SqlxResourceRepository,
};
use crate::repositories::{IContentRepository, InMemoryResourceRepository};
use crate::repositories::{IResourceRepository, SqlxAvatarRepository, SqlxContentRepository};
use anyhow::anyhow;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

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
    async fn get_resource<T>(
        &self,
        id: &ResourceID,
        lang: &Language,
        resource_type: &ResourceType,
    ) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned + Serialize;

    async fn list_resources<T>(
        &self,
        language: &Language,
        resource_type: &ResourceType,
        page: &Pagination,
    ) -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned + Serialize;

    /** Commit the transaction */
    async fn commit(mut self) -> anyhow::Result<()>;
    /** Rollback the transaction */
    async fn rollback(mut self) -> anyhow::Result<()>;
}

pub struct InMemory {
    error: bool,
    resource_repository: Option<InMemoryResourceRepository>,
    content_repository: Option<InMemoryContentRepository>,
    avatar_repository: Option<InMemoryAvatarRepository>,
}

#[cfg(test)]
impl InMemory {
    pub fn new() -> Self {
        Self {
            error: false,
            resource_repository: None,
            content_repository: None,
            avatar_repository: None,
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            resource_repository: self.resource_repository.map(|repo| repo.with_error()),
            content_repository: self.content_repository.map(|repo| repo.with_error()),
            avatar_repository: self.avatar_repository.map(|repo| repo.with_error()),
        }
    }
}

#[async_trait::async_trait]
impl IResourceUnitOfWork for InMemory {
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

    async fn get_resource<T>(
        &self,
        id: &ResourceID,
        lang: &Language,
        resource_type: &ResourceType,
    ) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned + Serialize,
    {
        let content_id = ContentID::from(id.clone());
        let data = self
            .content_repository
            .as_ref()
            .unwrap()
            .get(&content_id, lang)
            .await;

        match data {
            Ok(Some(data)) => {
                let json = match resource_type {
                    ResourceType::Member => {
                        let avatar = self.avatar_repository.as_ref().unwrap().get(id).await?;
                        let avatar = avatar.and_then(|json| {
                            serde_json::value::from_value::<AvatarData>(json.get()).ok()
                        });
                        let json = serde_json::from_value::<MemberData>(data.clone().to_json())?;
                        let member = MemberEntity::new(
                            id.clone().to_string(),
                            lang.as_str().to_string(),
                            json,
                            avatar,
                        );
                        serde_json::value::to_value(member)?
                    }
                    ResourceType::Service => {
                        let json = serde_json::from_value::<ServiceData>(data.clone().to_json())?;
                        let service = ServiceEntity::new(
                            id.clone().to_string(),
                            lang.as_str().to_string(),
                            json,
                        );

                        serde_json::value::to_value(service)?
                    }
                    ResourceType::Home => {
                        let json = serde_json::from_value::<HomeData>(data.clone().to_json())?;
                        let home = HomeEntity::new(
                            id.clone().to_string(),
                            lang.as_str().to_string(),
                            json,
                        );

                        serde_json::value::to_value(home)?
                    }
                    ResourceType::Contact => {
                        let json = serde_json::from_value::<ContactData>(data.clone().to_json())?;
                        let contact = ContactEntity::new(
                            id.clone().to_string(),
                            lang.as_str().to_string(),
                            json,
                        );

                        serde_json::value::to_value(contact)?
                    }
                    ResourceType::Article => {
                        let json = serde_json::from_value::<ArticleData>(data.clone().to_json())?;
                        let article = ArticleEntity::new(
                            id.clone().to_string(),
                            lang.as_str().to_string(),
                            json,
                        );

                        serde_json::value::to_value(article)?
                    }
                };

                let res = from_resource::<T>(json)?;
                Ok(Some(res))
            }
            Err(e) => Err(e),
            Ok(None) => Ok(None),
        }
    }

    async fn list_resources<T>(
        &self,
        language: &Language,
        resource_type: &ResourceType,
        _: &Pagination,
    ) -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned + Serialize,
    {
        let contents = self
            .content_repository
            .as_ref()
            .unwrap()
            .list(language)
            .await?;

        let mut res = vec![];

        for content in contents {
            let id = ResourceID::try_from(content.0.clone()).unwrap();
            let exist = self
                .resource_repository
                .as_ref()
                .unwrap()
                .contains(&id, resource_type)
                .await?;

            let entity = match resource_type {
                ResourceType::Member => {
                    if exist {
                        let data =
                            serde_json::value::from_value::<MemberData>(content.1.to_json())?;
                        Some(serde_json::value::to_value(SimpleMemberEntity::new(
                            content.0, data.name, None,
                        ))?)
                    } else {
                        None
                    }
                }
                ResourceType::Service => {
                    if exist {
                        let data =
                            serde_json::value::from_value::<ServiceData>(content.1.to_json())?;
                        Some(serde_json::value::to_value(ServiceEntity::new(
                            content.0,
                            language.as_str().to_string(),
                            data,
                        ))?)
                    } else {
                        None
                    }
                }
                _ => unimplemented!(),
            };

            res.push(entity);
        }

        let res = res
            .into_iter()
            .filter(|e| e.is_some())
            .map(|e| {
                let json = e.unwrap();
                from_resource(json).unwrap()
            })
            .collect::<Vec<_>>();

        Ok(res)
    }

    async fn commit(mut self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn rollback(mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

fn from_resource<T>(resource: serde_json::value::Value) -> Result<T, serde_json::error::Error>
where
    T: DeserializeOwned + Serialize,
{
    serde_json::value::from_value::<T>(resource)
}

#[derive(Debug)]
pub struct InDatabase<'tx> {
    pool: &'tx PgPool,
    tx: Arc<Mutex<Transaction<'tx, Postgres>>>,
    resource_repository: Option<SqlxResourceRepository<'tx>>,
    content_repository: Option<SqlxContentRepository<'tx>>,
    avatar_repository: Option<SqlxAvatarRepository<'tx>>,
}

impl<'tx> InDatabase<'tx> {
    pub async fn new(pool: &'tx PgPool) -> anyhow::Result<Self> {
        let tx = pool.begin().await?;
        let tx = Arc::new(Mutex::new(tx));

        Ok(Self {
            pool,
            tx,
            content_repository: None,
            avatar_repository: None,
            resource_repository: None,
        })
    }
}

#[async_trait::async_trait]
impl<'tx> IResourceUnitOfWork for InDatabase<'tx> {
    fn resource_repository(&mut self) -> &mut impl IResourceRepository {
        if self.resource_repository.is_none() {
            let resource_repo = SqlxResourceRepository::new(Arc::downgrade(&self.tx));
            self.resource_repository = Some(resource_repo);
        }
        self.resource_repository.as_mut().unwrap()
    }

    fn content_repository(&mut self) -> &mut impl IContentRepository {
        if self.content_repository.is_none() {
            // Use Arc::downgrade to obtain a weak reference to the transaction
            // if we don't do this, when we call the commit/rollback method will fail.
            // It can't `try_unwrap` because there are at least two strong references, preventing
            // the use of `try_unwrap`.
            //
            // If we want to use strong references, then we need to drop the repository
            // when we try to call commit/rollback methods.
            let content_repo = SqlxContentRepository::new(Arc::downgrade(&self.tx));
            self.content_repository = Some(content_repo);
        }
        self.content_repository.as_mut().unwrap()
    }

    fn avatar_repository(&mut self) -> &mut impl IAvatarRepository {
        if self.avatar_repository.is_none() {
            let avatar_repo = SqlxAvatarRepository::new(Arc::downgrade(&self.tx));
            self.avatar_repository = Some(avatar_repo);
        }
        self.avatar_repository.as_mut().unwrap()
    }

    async fn get_resource<T>(
        &self,
        id: &ResourceID,
        lang: &Language,
        resource_type: &ResourceType,
    ) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned + Serialize,
    {
        let query = r#"
                select resource.id as id,
                    content.data as data,
                    content.language as language
                from resource,
                     content
                where resource.id = content.id
                  and content.language = $2
                  and resource.id = $1
                  and resource.deleted_at is null;
                        "#;

        let res = match resource_type {
            ResourceType::Member => {
                let query = r#"
                select resource.id as id,
                    content.data as data,
                    avatar.data as avatar,
                    content.language as language
                from resource,
                     content
                         left join avatar on avatar.id = content.id
                where resource.id = content.id
                  and content.language = $2
                  and resource.id = $1
                  and resource.deleted_at is null;
                        "#;

                sqlx::query_as::<_, MemberEntityFromSQLx>(query)
                    .bind(id.as_str())
                    .bind(lang.as_str())
                    .fetch_optional(self.pool)
                    .await?
                    .map(MemberEntity::from)
                    .and_then(|e| serde_json::value::to_value(e).ok())
            }
            ResourceType::Service => sqlx::query_as::<_, ServiceEntityFromSQLx>(query)
                .bind(id.as_str())
                .bind(lang.as_str())
                .fetch_optional(self.pool)
                .await?
                .map(ServiceEntity::from)
                .and_then(|e| serde_json::value::to_value(e).ok()),
            ResourceType::Home => sqlx::query_as::<_, HomeEntityFromSQLx>(query)
                .bind(id.as_str())
                .bind(lang.as_str())
                .fetch_optional(self.pool)
                .await?
                .map(HomeEntity::from)
                .and_then(|e| serde_json::value::to_value(e).ok()),
            ResourceType::Contact => sqlx::query_as::<_, ContactEntityFromSQLx>(query)
                .bind(id.as_str())
                .bind(lang.as_str())
                .fetch_optional(self.pool)
                .await?
                .map(ContactEntity::from)
                .and_then(|e| serde_json::value::to_value(e).ok()),
            ResourceType::Article => sqlx::query_as::<_, ArticleEntityFromSQLx>(query)
                .bind(id.as_str())
                .bind(lang.as_str())
                .fetch_optional(self.pool)
                .await?
                .map(ArticleEntity::from)
                .and_then(|e| serde_json::value::to_value(e).ok()),
        };

        match res {
            None => Ok(None),
            Some(value) => {
                let r = from_resource::<T>(value)?;
                Ok(Some(r))
            }
        }
    }

    async fn list_resources<T>(
        &self,
        language: &Language,
        resource_type: &ResourceType,
        page: &Pagination,
    ) -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned + Serialize,
    {
        let offset = match page {
            Pagination::All => ";".to_string(),
            Pagination::Single => "limit 1;".to_string(),
            Pagination::Page(page) => {
                let offset = page.page * page.size;
                format!("offset {} limit {};", offset, page.size)
            }
        };

        let query = r#"select resource.id as id,
                content.data as data,
                content.language as language
                from resource,
                    content
                where resource.id = content.id
                and content.language = $1
                and resource.resource_type = $2
                and resource.deleted_at is null
                order by seq
                "#;

        let res = match resource_type {
            ResourceType::Member => {
                let query = r"select resource.id as id,
                content.data->>'name' as name,
                avatar.data->>'small_image' as avatar
                from resource,
                     content
                left join avatar on content.id = avatar.id
                where resource.id = content.id
                and content.language = $1
                and resource.resource_type = $2
                and resource.deleted_at is null
                order by seq";
                let query = format!("{}{}", query, offset);

                sqlx::query_as::<_, SimpleMemberEntityFromSQLx>(query.as_str())
                    .bind(language.as_str())
                    .bind(resource_type.as_str())
                    .fetch_all(self.pool)
                    .await?
                    .into_iter()
                    .map(SimpleMemberEntity::from)
                    .filter_map(|e| serde_json::value::to_value(e).ok())
                    .collect::<Vec<_>>()
            }
            ResourceType::Service => {
                let query = format!("{}{}", query, offset);

                sqlx::query_as::<_, ServiceEntityFromSQLx>(query.as_str())
                    .bind(language.as_str())
                    .bind(resource_type.as_str())
                    .fetch_all(self.pool)
                    .await?
                    .into_iter()
                    .map(ServiceEntity::from)
                    .filter_map(|e| serde_json::value::to_value(e).ok())
                    .collect::<Vec<_>>()
            }
            ResourceType::Home => {
                let query = format!("{}{}", query, offset);

                sqlx::query_as::<_, HomeEntityFromSQLx>(query.as_str())
                    .bind(language.as_str())
                    .bind(resource_type.as_str())
                    .fetch_all(self.pool)
                    .await?
                    .into_iter()
                    .map(HomeEntity::from)
                    .filter_map(|e| serde_json::value::to_value(e).ok())
                    .collect::<Vec<_>>()
            }
            ResourceType::Article => {
                let query = format!("{}{}", query, offset);

                sqlx::query_as::<_, ArticleEntityFromSQLx>(query.as_str())
                    .bind(language.as_str())
                    .bind(resource_type.as_str())
                    .fetch_all(self.pool)
                    .await?
                    .into_iter()
                    .map(ArticleEntity::from)
                    .filter_map(|e| serde_json::value::to_value(e).ok())
                    .collect::<Vec<_>>()
            }
            ResourceType::Contact => {
                let query = format!("{}{}", query, offset);

                sqlx::query_as::<_, ContactEntityFromSQLx>(query.as_str())
                    .bind(language.as_str())
                    .bind(resource_type.as_str())
                    .fetch_all(self.pool)
                    .await?
                    .into_iter()
                    .map(ContactEntity::from)
                    .filter_map(|e| serde_json::value::to_value(e).ok())
                    .collect::<Vec<_>>()
            }
        };

        Ok(res
            .into_iter()
            .map(from_resource::<T>)
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>())
    }

    async fn commit(self) -> anyhow::Result<()> {
        match Arc::try_unwrap(self.tx) {
            Ok(lock) => {
                lock.into_inner().commit().await?;
                Ok(())
            }
            Err(_) => Err(anyhow!("can't commit transaction")),
        }
    }

    async fn rollback(self) -> anyhow::Result<()> {
        match Arc::try_unwrap(self.tx) {
            Ok(lock) => {
                lock.into_inner().rollback().await?;
                Ok(())
            }
            Err(_) => Err(anyhow!("can't rollback transaction")),
        }
    }
}
