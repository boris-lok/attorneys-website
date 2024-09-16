use crate::domain::home::entities::{Home, HomeID};
use crate::domain::member::entities::Language;
use crate::uow::home::IHomeUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) home_id: String,
    pub(crate) language: String,
    pub(crate) default_language: Language,
}

#[derive(Debug)]
pub(crate) enum Error {
    BadRequest,
    HomeNotFound,
    Unknown,
}

pub async fn execute<IUnitOfWork>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<Option<Home>, Error>
where
    IUnitOfWork: IHomeUnitOfWork,
{
    let mut lock = uow.lock().await;
    let home_id = HomeID::try_from(req.home_id).map_err(|_| Error::BadRequest)?;
    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let res = lock
        .get_home(&home_id, &language)
        .await
        .map_err(|_| Error::Unknown)?;

    match res {
        None => {
            if language != req.default_language {
                match lock.get_home(&home_id, &req.default_language).await {
                    Ok(Some(res)) => Ok(Some(res)),
                    Ok(None) => Err(Error::HomeNotFound),
                    Err(_) => Err(Error::Unknown),
                }
            } else {
                Err(Error::HomeNotFound)
            }
        }
        Some(res) => Ok(Some(res)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::home::entities::HomeData;
    use crate::domain::member::entities::ContentData;
    use crate::domain::test_helper::create_fake_home_helper;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_a_home_otherwise() {
        let home_id = HomeID::try_from(Ulid::new().to_string()).unwrap();
        let content = HomeData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow = create_fake_home_helper(home_id.clone(), Some(data), Language::EN, false).await;

        let req = Request {
            home_id: home_id.as_str().to_string(),
            language: "en".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(Some(service)) => {
                assert_eq!(service.home_id, home_id.as_str());
                assert_eq!(service.content, "data");
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_a_default_language_home_otherwise() {
        let home_id = HomeID::try_from(Ulid::new().to_string()).unwrap();
        let content = HomeData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow = create_fake_home_helper(home_id.clone(), Some(data), Language::EN, false).await;

        let req = Request {
            home_id: home_id.as_str().to_string(),
            language: "zh".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(Some(service)) => {
                assert_eq!(service.home_id, home_id.as_str());
                assert_eq!(service.content, "data");
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_home_is_not_found() {
        let home_id = HomeID::try_from(Ulid::new().to_string()).unwrap();
        let content = HomeData {
            data: "data".to_string(),
        };
        let data = ContentData::try_from(content).unwrap();

        let uow = create_fake_home_helper(home_id.clone(), Some(data), Language::EN, false).await;

        let req = Request {
            home_id: "not_found_id".to_string(),
            language: "en".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::HomeNotFound) => {}
            _ => unreachable!(),
        }
    }
}
