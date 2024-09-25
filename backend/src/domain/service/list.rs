use crate::domain::member::entities::Language;
use crate::domain::service::entities::Service;
use crate::uow::service::IServiceUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) language: String,
    pub(crate) default_language: Language,
}

pub(crate) enum Error {
    BadRequest,
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<Vec<Service>, Error>
where
    IUnitOfWork: IServiceUnitOfWork,
{
    let mut lock = uow.lock().await;

    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let res = lock.get_all_services_by_language(&language).await;

    match res {
        Ok(services) => {
            if services.is_empty() {
                match lock
                    .get_all_services_by_language(&req.default_language)
                    .await
                {
                    Ok(services) => Ok(services),
                    Err(e) => Err(Error::Unknown(e.to_string())),
                }
            } else {
                Ok(services)
            }
        }
        Err(e) => Err(Error::Unknown(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::member::entities::ContentData;
    use crate::domain::service::entities::{ServiceData, ServiceID};
    use crate::domain::test_helper::create_fake_service_helper;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_an_empty_list_otherwise() {
        let mut uow = crate::uow::service::InMemoryServiceUnitOfWork::new();
        let _ = uow.content_repository();

        let req = Request {
            language: "en".to_string(),
            default_language: Language::ZH,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(list) => {
                assert_eq!(list.len(), 0);
            }
            Err(_) => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_a_services_list_otherwise() {
        let service_id = ServiceID::try_from(Ulid::new().to_string()).unwrap();
        let content = ServiceData {
            data: "data".to_string(),
        };

        let uow = create_fake_service_helper(
            service_id.clone(),
            Some(ContentData::try_from(content).unwrap()),
            Language::EN,
            false,
        )
            .await;

        let req = Request {
            language: "en".to_string(),
            default_language: Language::ZH,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(list) => {
                assert_eq!(list.len(), 1);
            }
            Err(_) => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_a_services_list_with_default_language_otherwise() {
        let service_id = ServiceID::try_from(Ulid::new().to_string()).unwrap();
        let content = ServiceData {
            data: "data".to_string(),
        };
        let uow = create_fake_service_helper(
            service_id.clone(),
            Some(ContentData::try_from(content).unwrap()),
            Language::ZH,
            false,
        )
            .await;

        let req = Request {
            language: "en".to_string(),
            default_language: Language::ZH,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(list) => {
                assert_eq!(list.len(), 1);
            }
            Err(_) => unreachable!(),
        };
    }
}
