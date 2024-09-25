use crate::domain::home::entities::Home;
use crate::domain::member::entities::Language;
use crate::uow::home::IHomeUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) language: String,
    pub(crate) default_language: Language,
}

pub(crate) enum Error {
    BadRequest,
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<Vec<Home>, Error>
where
    IUnitOfWork: IHomeUnitOfWork,
{
    let mut lock = uow.lock().await;

    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let res = lock.get_all_home_by_language(&language).await;

    match res {
        Ok(home) => {
            if home.is_empty() {
                match lock.get_all_home_by_language(&req.default_language).await {
                    Ok(services) => Ok(services),
                    Err(e) => Err(Error::Unknown(e.to_string())),
                }
            } else {
                Ok(home)
            }
        }
        Err(e) => Err(Error::Unknown(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::home::entities::{HomeData, HomeID};
    use crate::domain::member::entities::ContentData;
    use crate::domain::test_helper::create_fake_home_helper;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_an_empty_list_otherwise() {
        let mut uow = crate::uow::home::InMemoryHomeUnitOfWork::new();
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
    async fn it_should_return_a_home_list_otherwise() {
        let home_id = HomeID::try_from(Ulid::new().to_string()).unwrap();
        let content = HomeData {
            data: "data".to_string(),
        };

        let uow = create_fake_home_helper(
            home_id.clone(),
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
    async fn it_should_return_a_home_list_with_default_language_otherwise() {
        let service_id = HomeID::try_from(Ulid::new().to_string()).unwrap();
        let content = HomeData {
            data: "data".to_string(),
        };
        let uow = create_fake_home_helper(
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
