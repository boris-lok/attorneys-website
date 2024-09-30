use crate::domain::entities::{Language, ResourceID, ResourceRecord, ResourceType};
use crate::uow::IResourceUnitOfWork;
use serde::de::DeserializeOwned;
use std::fmt::Formatter;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) id: String,
    pub(crate) resource_type: ResourceType,
    pub(crate) language: String,
    pub(crate) default_language: Language,
}

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
                write!(f, "{}", e.to_string())
            }
        }
    }
}

pub async fn execute<IUnitOfWork>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<ResourceRecord, Error>
where
    IUnitOfWork: IResourceUnitOfWork,
{
    async fn inner_execute<IUnitOfWork>(
        uow: Arc<Mutex<IUnitOfWork>>,
        id: &ResourceID,
        lang: &Language,
        resource_type: &ResourceType,
    ) -> Result<ResourceRecord, Error>
    where
        IUnitOfWork: IResourceUnitOfWork,
    {
        let mut lock = uow.lock().await;
        match lock.get_resource(&id, lang, resource_type).await {
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
        ContactData, ContentData, ContentID, HomeData, MemberData, Resource, ServiceData,
    };
    use crate::repositories::IContentRepository;
    use crate::uow::InMemoryResource;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_a_resource_otherwise() {
        let member = MemberData::new("boris".to_string(), "description".to_string());
        let service = ServiceData::new("title".to_string(), "data".to_string());
        let home = HomeData::new("home".to_string());
        let contact = ContactData::new(
            "address".to_string(),
            "1234".to_string(),
            "info@example.com".to_string(),
        );
        let testcases = vec![
            (
                Ulid::new().to_string(),
                ResourceType::Member,
                Resource::Member(member.clone()),
            ),
            (
                Ulid::new().to_string(),
                ResourceType::Service,
                Resource::Service(service.clone()),
            ),
            (
                Ulid::new().to_string(),
                ResourceType::Home,
                Resource::Home(home.clone()),
            ),
            (
                Ulid::new().to_string(),
                ResourceType::Contact,
                Resource::Contact(contact.clone()),
            ),
        ];

        for (id, resource_type, resource) in testcases {
            let content_data = ContentData::try_from(resource.clone()).unwrap();
            let resource_id = ResourceID::try_from(id.clone()).unwrap();
            let content_id = ContentID::from(resource_id);

            let mut uow = InMemoryResource::new();

            let repo = uow
                .content_repository()
                .insert(content_id.clone(), content_data, Language::ZH)
                .await
                .unwrap();

            let req = Request {
                id: id.clone(),
                resource_type: resource_type.clone(),
                language: "zh".to_string(),
                default_language: Language::ZH,
            };

            let res = execute(Mutex::new(uow), req).await;

            match res {
                Ok(record) => match (record.resource, resource_type) {
                    (Resource::Member(m), ResourceType::Member) => assert_eq!(m, member),
                    (Resource::Service(s), ResourceType::Service) => assert_eq!(s, service),
                    (Resource::Home(h), ResourceType::Home) => assert_eq!(h, home),
                    (Resource::Contact(c), ResourceType::Contact) => assert_eq!(c, contact),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }
}
