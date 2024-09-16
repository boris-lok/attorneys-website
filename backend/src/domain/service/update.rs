use crate::domain::member::entities::{ContentData, ContentID, Language};
use crate::domain::service::entities::{ServiceData, ServiceID};
use crate::repositories::content_repository::IContentRepository;
use crate::repositories::service_repository::IServiceRepository;
use crate::uow::member::IMemberUnitOfWork;
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

pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<(), Error>
where
    IUnitOfWork: IServiceUnitOfWork,
{
    let res = {
        let mut lock = uow.lock().await;
        let (service_id, data, language) = match (
            ServiceID::try_from(req.service_id),
            ServiceData::try_from(req.data),
            Language::try_from(req.language),
        ) {
            (Ok(member_id), Ok(data), Ok(language)) => (member_id, data, language),
            _ => return Err(Error::BadRequest),
        };

        match lock.service_repository().contains(&service_id).await {
            Ok(exist) if !exist => return Err(Error::BadRequest),
            Err(e) => return Err(Error::Unknown(e.to_string())),
            Ok(_) => {}
        };
        let content_id = ContentID::try_from(service_id)
            .map_err(|_| Error::Unknown("Can't parse member_id to content_id".to_string()))?;
        let data = ContentData::try_from(data)
            .map_err(|_| Error::Unknown("Can't parse data to json".to_string()))?;

        match lock
            .content_repository()
            .update(&content_id, data, language)
            .await
        {
            Ok(_) => {}
            Err(e) => return Err(Error::Unknown(e.to_string())),
        };
    };

    uow.into_inner()
        .commit()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::test_helper::{create_fake_member_helper, create_fake_service_helper};
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_success_otherwise() {
        let service_id = ServiceID::try_from(Ulid::new().to_string()).unwrap();
        let content = ServiceData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow =
            create_fake_service_helper(service_id.clone(), Some(data), Language::EN, false).await;

        let req = Request {
            service_id: service_id.to_string(),
            data: "new data".to_string(),
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(_) => {
                // TODO: check the data is updated
            }
            Err(_) => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_member_does_not_exist() {
        let service_id = ServiceID::try_from(Ulid::new().to_string()).unwrap();
        let content = ServiceData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow =
            create_fake_service_helper(service_id.clone(), Some(data), Language::EN, false).await;

        let not_exists_id = Ulid::new().to_string();

        let req = Request {
            service_id: not_exists_id,
            data: "new data".to_string(),
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_unexpected_error_encountered() {
        let service_id = ServiceID::try_from(Ulid::new().to_string()).unwrap();
        let content = ServiceData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow =
            create_fake_service_helper(service_id.clone(), Some(data), Language::EN, true).await;

        let req = Request {
            service_id: service_id.to_string(),
            data: "new data".to_string(),
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        }
    }
}
