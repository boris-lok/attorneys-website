use crate::domain::entities::{Language, ResourceID};
use crate::uow::IResourceUnitOfWork;
use serde::de::DeserializeOwned;
use std::fmt::Formatter;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) id: String,
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
        write!(f, "{}", self)
    }
}

pub async fn execute<IUnitOfWork, T>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<Option<T>, Error>
where
    IUnitOfWork: IResourceUnitOfWork,
    T: DeserializeOwned,
{
    async fn inner_execute<IUnitOfWork, T>(
        uow: Arc<Mutex<IUnitOfWork>>,
        id: &ResourceID,
        lang: &Language,
    ) -> Result<Option<T>, Error>
    where
        IUnitOfWork: IResourceUnitOfWork,
        T: DeserializeOwned,
    {
        let mut lock = uow.lock().await;
        match lock.get_resource(&id, lang).await {
            Ok(Some(res)) => Ok(Some(res)),
            Ok(None) => Err(Error::NotFound),
            Err(e) => Err(Error::Unknown(e.to_string())),
        }
    }

    let id = ResourceID::try_from(req.id).map_err(|_| Error::BadRequest)?;
    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let uow = Arc::new(uow);

    match inner_execute(uow.clone(), &id, &language).await {
        Ok(Some(res)) => Ok(res),
        Ok(None) => {
            return inner_execute(uow.clone(), &id, &req.default_language).await;
        }
        Err(e) => Err(Error::Unknown(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{ContentData, ContentID, MemberData, Resource};
    use crate::repositories::IContentRepository;
    use crate::uow::InMemoryResource;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_a_member_otherwise() {
        let mut uow = InMemoryResource::new();
        let data = Resource::Member(MemberData::new(
            "boris".to_string(),
            "description".to_string(),
        ));
        let content_data = ContentData::try_from(data).unwrap();

        let id = Ulid::new().to_string();
        let resource_id = ResourceID::try_from(id.clone()).unwrap();
        let content_id = ContentID::from(resource_id);

        let repo = uow
            .content_repository()
            .insert(content_id.clone(), content_data, Language::ZH)
            .await
            .unwrap();

        let req = Request {
            id: id.clone(),
            language: "zh".to_string(),
            default_language: Language::ZH,
        };

        let res: Result<Option<MemberData>, Error> = execute(Mutex::new(uow), req).await;

        match res {
            Ok(Some(m)) => {
                assert_eq!(m.name, "boris");
                assert_eq!(m.description, "description");
            }
            _ => unreachable!(),
        }
    }
}
