use crate::domain::entities::{Language, ResourceID, ResourceType};
use crate::uow::IResourceUnitOfWork;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Formatter;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) id: String,
    pub(crate) resource_type: ResourceType,
    pub(crate) language: String,
    pub(crate) default_language: Language,
}

#[derive(Debug)]
pub(crate) enum Error {
    BadRequest,
    NotFound,
    Unknown(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequest => {
                write!(f, "Bad request")
            }
            Error::NotFound => {
                write!(f, "Not found")
            }
            Error::Unknown(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

pub async fn execute<IUnitOfWork, T>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<T, Error>
where
    IUnitOfWork: IResourceUnitOfWork,
    T: DeserializeOwned + Serialize,
{
    async fn inner_execute<IUnitOfWork, T>(
        uow: Arc<Mutex<IUnitOfWork>>,
        id: &ResourceID,
        lang: &Language,
        resource_type: &ResourceType,
    ) -> Result<T, Error>
    where
        IUnitOfWork: IResourceUnitOfWork,
        T: DeserializeOwned + Serialize,
    {
        let mut lock = uow.lock().await;
        match lock.get_resource(id, lang, resource_type).await {
            Ok(Some(res)) => Ok(res),
            Ok(None) => Err(Error::NotFound),
            Err(e) => Err(Error::Unknown(e.to_string())),
        }
    }

    let id = ResourceID::try_from(req.id).map_err(|_| Error::BadRequest)?;
    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let uow = Arc::new(uow);

    match inner_execute(uow.clone(), &id, &language, &req.resource_type).await {
        Ok(res) => Ok(res),
        Err(Error::NotFound) => {
            inner_execute(uow.clone(), &id, &req.default_language, &req.resource_type).await
        }
        Err(e) => Err(Error::Unknown(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        ContactData, ContactEntity, ContentData, ContentID, HomeData, HomeEntity, MemberData,
        MemberEntity, Resource, ServiceData, ServiceEntity,
    };
    use crate::domain::member::entities::{AvatarData, AvatarJson};
    use crate::repositories::IAvatarRepository;
    use crate::repositories::IContentRepository;
    use crate::uow::InMemory;
    use ulid::Ulid;

    fn create_testcases() -> Vec<(String, ResourceType, Resource, Option<AvatarData>)> {
        let member = MemberData::new("boris".to_string(), "description".to_string());
        let service = ServiceData::new("title".to_string(), "data".to_string());
        let home = HomeData::new("home".to_string());
        let contact = ContactData::new(
            "address".to_string(),
            "1234".to_string(),
            "info@example.com".to_string(),
        );
        let avatar = AvatarData {
            large_image: "large".to_string(),
            small_image: "small".to_string(),
        };
        vec![
            (
                Ulid::new().to_string(),
                ResourceType::Member,
                Resource::Member(member.clone()),
                Some(avatar),
            ),
            (
                Ulid::new().to_string(),
                ResourceType::Member,
                Resource::Member(member.clone()),
                None,
            ),
            (
                Ulid::new().to_string(),
                ResourceType::Service,
                Resource::Service(service.clone()),
                None,
            ),
            (
                Ulid::new().to_string(),
                ResourceType::Home,
                Resource::Home(home.clone()),
                None,
            ),
            (
                Ulid::new().to_string(),
                ResourceType::Contact,
                Resource::Contact(contact.clone()),
                None,
            ),
        ]
    }

    async fn create_a_fake_resource_and_return_the_unit_of_work(
        id: String,
        resource: Resource,
        avatar: Option<AvatarData>,
    ) -> InMemory {
        let content_data = ContentData::try_from(resource).unwrap();
        let resource_id = ResourceID::try_from(id.clone()).unwrap();
        let content_id = ContentID::from(resource_id.clone());

        let mut uow = InMemory::new();

        let repo = uow
            .content_repository()
            .insert(content_id.clone(), content_data, Language::ZH)
            .await
            .unwrap();

        match avatar {
            Some(avatar) => {
                let json = AvatarJson::try_from(avatar).unwrap();
                uow.avatar_repository()
                    .insert(resource_id.clone(), json)
                    .await
                    .unwrap();
            }
            None => {
                // Workaround: touch the repository for initialization
                let _ = uow.avatar_repository();
            }
        }
        // Workaround: touch the repository for initialization
        let _ = uow.resource_repository();

        uow
    }

    async fn act_and_assert_successfully(
        uow: InMemory,
        req: Request,
        resource: Resource,
        avatar: Option<AvatarData>,
    ) {
        match resource {
            Resource::Member(m) => {
                let res: MemberEntity = execute(Mutex::new(uow), req)
                    .await
                    .expect("should execute successfully");

                assert_eq!(res.data, m);
                assert_eq!(avatar, res.avatar);
            }
            Resource::Service(s) => {
                let res: ServiceEntity = execute(Mutex::new(uow), req)
                    .await
                    .expect("should execute successfully");
                assert_eq!(res.data, s)
            }
            Resource::Home(h) => {
                let res: HomeEntity = execute(Mutex::new(uow), req)
                    .await
                    .expect("should execute successfully");
                assert_eq!(res.data, h)
            }
            Resource::Contact(c) => {
                let res: ContactEntity = execute(Mutex::new(uow), req)
                    .await
                    .expect("should execute successfully");
                assert_eq!(res.data, c)
            }
        }
    }

    #[tokio::test]
    async fn it_should_return_a_resource_otherwise() {
        let testcases = create_testcases();

        for (id, resource_type, resource, avatar) in testcases {
            let uow = create_a_fake_resource_and_return_the_unit_of_work(
                id.clone(),
                resource.clone(),
                avatar.clone(),
            )
            .await;

            let req = Request {
                id: id.clone(),
                resource_type: resource_type.clone(),
                language: "zh".to_string(),
                default_language: Language::ZH,
            };

            act_and_assert_successfully(uow, req, resource, avatar).await;
        }
    }

    #[tokio::test]
    async fn it_should_return_a_resource_with_default_language_otherwise() {
        let testcases = create_testcases();

        for (id, resource_type, resource, avatar) in testcases {
            let uow = create_a_fake_resource_and_return_the_unit_of_work(
                id.clone(),
                resource.clone(),
                avatar.clone(),
            )
            .await;

            let req = Request {
                id: id.clone(),
                resource_type: resource_type.clone(),
                language: "en".to_string(),
                default_language: Language::ZH,
            };

            act_and_assert_successfully(uow, req, resource, avatar).await;
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_unexpected_error_encountered() {
        let testcases = create_testcases();

        for (id, resource_type, resource, avatar) in testcases {
            let uow = create_a_fake_resource_and_return_the_unit_of_work(
                id.clone(),
                resource.clone(),
                avatar,
            )
            .await
            .with_error();

            let req = Request {
                id: id.clone(),
                resource_type: resource_type.clone(),
                language: "zh".to_string(),
                default_language: Language::ZH,
            };

            match resource {
                Resource::Member(m) => {
                    let res: Result<MemberEntity, Error> = execute(Mutex::new(uow), req).await;

                    assert!(res.is_err());
                }
                Resource::Service(s) => {
                    let res: Result<ServiceEntity, Error> = execute(Mutex::new(uow), req).await;
                    assert!(res.is_err());
                }
                Resource::Home(h) => {
                    let res: Result<HomeEntity, Error> = execute(Mutex::new(uow), req).await;
                    assert!(res.is_err());
                }
                Resource::Contact(c) => {
                    let res: Result<ContactEntity, Error> = execute(Mutex::new(uow), req).await;
                    assert!(res.is_err());
                }
            }
        }
    }
}
