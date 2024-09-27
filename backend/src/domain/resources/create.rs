use crate::domain::entities::{ContentData, ContentID, Language, ResourceID, ResourceType};
use crate::repositories::IContentRepository;
use crate::repositories::IResourceRepository;
use crate::uow::IResourceUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request<T> {
    pub(crate) id: String,
    pub(crate) data: T,
    pub(crate) language: String,
    pub(crate) resource_type: ResourceType,
}

pub(crate) enum Error {
    BadRequest,
    Unknown(String),
}

pub async fn execute<IUnitOfWork, Data>(
    uow: Mutex<IUnitOfWork>,
    req: Request<Data>,
) -> Result<ContentID, Error>
where
    IUnitOfWork: IResourceUnitOfWork,
    ContentData: TryFrom<Data>,
{
    let id = {
        let mut lock = uow.lock().await;
        let (id, data, language) = match (
            ResourceID::try_from(req.id),
            ContentData::try_from(req.data),
            Language::try_from(req.language),
        ) {
            (Ok(id), Ok(data), Ok(language)) => (id, data, language),
            _ => return Err(Error::BadRequest),
        };

        let content_id = match lock
            .resource_repository()
            .insert(id, req.resource_type)
            .await
        {
            Ok(id) => ContentID::from(id),
            Err(e) => return Err(Error::Unknown(e.to_string())),
        };

        match lock
            .content_repository()
            .insert(content_id, data, language)
            .await
        {
            Ok(id) => id,
            Err(e) => return Err(Error::Unknown(e.to_string())),
        }
    };

    uow.into_inner()
        .commit()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    Ok(id)
}
