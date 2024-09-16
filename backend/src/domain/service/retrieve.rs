use crate::domain::member::entities::Language;
use crate::domain::service::entities::{Service, ServiceID};
use crate::uow::service::IServiceUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) service_id: String,
    pub(crate) language: String,
    pub(crate) default_language: Language,
}

#[derive(Debug)]
pub(crate) enum Error {
    BadRequest,
    ServiceNotFound,
    Unknown,
}

pub async fn execute<IUnitOfWork>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<Option<Service>, Error>
where
    IUnitOfWork: IServiceUnitOfWork,
{
    let mut lock = uow.lock().await;
    let service_id = ServiceID::try_from(req.service_id).map_err(|_| Error::BadRequest)?;
    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let res = lock
        .get_service(&service_id, &language)
        .await
        .map_err(|_| Error::Unknown)?;

    match res {
        None => {
            if language != req.default_language {
                match lock.get_service(&service_id, &req.default_language).await {
                    Ok(Some(res)) => Ok(Some(res)),
                    Ok(None) => Err(Error::ServiceNotFound),
                    Err(_) => Err(Error::Unknown),
                }
            } else {
                Err(Error::ServiceNotFound)
            }
        }
        Some(res) => Ok(Some(res)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::member::entities::ContentData;
    use crate::domain::service::entities::ServiceData;
    use crate::domain::test_helper::create_fake_service_helper;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_a_member_otherwise() {
        let service_id = ServiceID::try_from(Ulid::new().to_string()).unwrap();
        let content = ServiceData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow =
            create_fake_service_helper(service_id.clone(), Some(data), Language::EN, false).await;

        let req = Request {
            service_id: service_id.as_str().to_string(),
            language: "en".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(Some(service)) => {
                assert_eq!(service.service_id, service_id.as_str());
                assert_eq!(service.content, "data");
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_a_default_language_member_otherwise() {
        let service_id = ServiceID::try_from(Ulid::new().to_string()).unwrap();
        let content = ServiceData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow =
            create_fake_service_helper(service_id.clone(), Some(data), Language::EN, false).await;

        let req = Request {
            service_id: service_id.as_str().to_string(),
            language: "zh".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(Some(service)) => {
                assert_eq!(service.service_id, service_id.as_str());
                assert_eq!(service.content, "data");
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_member_is_not_found() {
        let service_id = ServiceID::try_from(Ulid::new().to_string()).unwrap();
        let content = ServiceData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow =
            create_fake_service_helper(service_id.clone(), Some(data), Language::EN, false).await;

        let req = Request {
            service_id: "not_found_id".to_string(),
            language: "en".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::ServiceNotFound) => {}
            _ => unreachable!(),
        }
    }
}
