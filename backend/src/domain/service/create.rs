use crate::domain::member::entities::{ContentData, ContentID, Language};
use crate::domain::service::entities::{ServiceData, ServiceID};
use crate::repositories::content_repository::IContentRepository;
use crate::repositories::service_repository::IServiceRepository;
use crate::uow::service::IServiceUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) service_id: String,
    pub(crate) data: String,
    pub(crate) language: String,
}

#[derive(Debug)]
pub(crate) enum Error {
    BadRequest,
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<String, Error>
where
    IUnitOfWork: IServiceUnitOfWork,
{
    let mut lock = uow.lock().await;

    let (service_id, data, language) = match (
        ServiceID::try_from(req.service_id),
        ServiceData::try_from(req.data),
        Language::try_from(req.language),
    ) {
        (Ok(service_id), Ok(data), Ok(language)) => (service_id, data, language),
        _ => return Err(Error::BadRequest),
    };

    let content_id = match lock.service_repository().insert(service_id).await {
        Ok(id) => Ok(ContentID::try_from(id).unwrap()),
        Err(e) => return Err(Error::Unknown(e.to_string())),
    }?;

    let data = ContentData::try_from(data)
        .map_err(|_| Error::Unknown("Can't parse data to json".to_string()))?;

    let content_id = match lock
        .content_repository()
        .insert(content_id, data, language)
        .await
    {
        Ok(id) => Ok(id),
        Err(e) => return Err(Error::Unknown(e.to_string())),
    }?;

    drop(lock);
    uow.into_inner()
        .commit()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(content_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_the_service_id_otherwise() {
        let uow = crate::uow::service::InMemoryServiceUnitOfWork::new();
        let service_id = Ulid::new().to_string();
        let req = Request {
            service_id: service_id.clone(),
            data: "data".to_string(),
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(id) => assert_eq!(id, service_id),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_bad_request_error_when_request_is_invalid() {
        let uow = crate::uow::service::InMemoryServiceUnitOfWork::new();
        let service_id = Ulid::new().to_string();

        let req = Request {
            service_id,
            data: "".to_string(),
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let uow = crate::uow::service::InMemoryServiceUnitOfWork::new();
        let uow = uow.with_error();
        let service_id = Ulid::new().to_string();
        let req = Request {
            service_id,
            data: "data".to_string(),
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        }
    }
}
