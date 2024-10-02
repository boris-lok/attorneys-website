use crate::domain::entities::{Language, Pagination, ResourceType};
use crate::uow::IResourceUnitOfWork;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) struct Request {
    pub(crate) resource_type: ResourceType,
    pub(crate) language: String,
    pub(crate) default_language: Language,
    pub(crate) pagination: Pagination,
}

pub(crate) enum Error {
    BadRequest,
    Unknown(String),
}

pub async fn execute<IUnitOfWork, T>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<Vec<T>, Error>
where
    IUnitOfWork: IResourceUnitOfWork,
    T: DeserializeOwned + Serialize,
{
    async fn inner_execute<IUnitOfWork, T>(
        uow: Arc<Mutex<IUnitOfWork>>,
        lang: &Language,
        resource_type: &ResourceType,
        pagination: &Pagination,
    ) -> Result<Vec<T>, Error>
    where
        IUnitOfWork: IResourceUnitOfWork,
        T: DeserializeOwned + Serialize,
    {
        let lock = uow.lock().await;
        match lock
            .list_resources::<T>(lang, resource_type, pagination)
            .await
        {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::Unknown(e.to_string())),
        }
    }

    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let uow = Arc::new(uow);

    match inner_execute(uow.clone(), &language, &req.resource_type, &req.pagination).await {
        Ok(res) => {
            if res.is_empty() {
                inner_execute(
                    uow.clone(),
                    &req.default_language,
                    &req.resource_type,
                    &req.pagination,
                )
                .await
            } else {
                Ok(res)
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        ArticleData, ContactData, ContentData, ContentID, HomeData, MemberData, Resource,
        ResourceID, ServiceData, SimpleMemberEntity,
    };
    use crate::repositories::IContentRepository;
    use crate::repositories::IResourceRepository;
    use crate::uow::InMemory;

    fn create_a_resource() -> Vec<Resource> {
        let member = MemberData::new("boris".to_string(), "description".to_string());
        let service = ServiceData::new("title".to_string(), "data".to_string());
        let home = HomeData::new("home".to_string());
        let contact = ContactData::new(
            "address".to_string(),
            "1234".to_string(),
            "info@example.com".to_string(),
        );
        let article = ArticleData::new("title".to_string(), "data".to_string());

        vec![
            Resource::Article(article),
            Resource::Service(service),
            Resource::Home(home),
            Resource::Member(member),
            Resource::Contact(contact),
        ]
    }
    async fn create_some_fake_data_and_return_uow() -> InMemory {
        let resources = create_a_resource();

        let mut uow = InMemory::new();

        for resource in resources {
            let id = ulid::Ulid::new().to_string();
            let resource_id = ResourceID::try_from(id.clone()).unwrap();
            let content_id = ContentID::from(resource_id.clone());
            let content_data = ContentData::try_from(resource.clone()).unwrap();

            match resource {
                Resource::Member(_) => {
                    uow.resource_repository()
                        .insert(resource_id, ResourceType::Member)
                        .await
                        .unwrap();
                }
                Resource::Service(_) => {
                    uow.resource_repository()
                        .insert(resource_id, ResourceType::Service)
                        .await
                        .unwrap();
                }
                Resource::Home(_) => {
                    uow.resource_repository()
                        .insert(resource_id, ResourceType::Home)
                        .await
                        .unwrap();
                }
                Resource::Contact(_) => {
                    uow.resource_repository()
                        .insert(resource_id, ResourceType::Contact)
                        .await
                        .unwrap();
                }
                Resource::Article(_) => {
                    uow.resource_repository()
                        .insert(resource_id, ResourceType::Article)
                        .await
                        .unwrap();
                }
            };

            uow.content_repository()
                .insert(content_id.clone(), content_data, Language::ZH)
                .await
                .unwrap();
        }

        let _ = uow.avatar_repository();

        uow
    }

    #[tokio::test]
    async fn it_should_list_resource_otherwise() {
        let uow = create_some_fake_data_and_return_uow().await;

        let req = Request {
            resource_type: ResourceType::Member,
            language: "zh".to_string(),
            default_language: Language::ZH,
            pagination: Pagination::All,
        };

        let res = execute::<InMemory, SimpleMemberEntity>(Mutex::new(uow), req).await;

        match res {
            Ok(list) => {
                assert_eq!(list.len(), 1);
            }
            Err(_) => unreachable!(),
        }
    }
    #[tokio::test]
    async fn it_should_list_default_language_resource_otherwise() {
        let uow = create_some_fake_data_and_return_uow().await;

        let req = Request {
            resource_type: ResourceType::Member,
            language: "en".to_string(),
            default_language: Language::ZH,
            pagination: Pagination::All,
        };

        let res = execute::<InMemory, SimpleMemberEntity>(Mutex::new(uow), req).await;

        match res {
            Ok(list) => {
                assert_eq!(list.len(), 1);
            }
            Err(_) => unreachable!(),
        }
    }
}
