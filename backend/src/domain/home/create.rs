use crate::domain::home::entities::{HomeData, HomeID};
use crate::domain::member::entities::{ContentData, ContentID, Language};
use crate::repositories::content_repository::IContentRepository;
use crate::repositories::home_repository::IHomeRepository;
use crate::uow::home::IHomeUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) home_id: String,
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
    IUnitOfWork: IHomeUnitOfWork,
{
    let home_id = {
        let mut lock = uow.lock().await;

        let (home_id, data, language) = match (
            HomeID::try_from(req.home_id),
            HomeData::try_from(req.data),
            Language::try_from(req.language),
        ) {
            (Ok(home_id), Ok(data), Ok(language)) => (home_id, data, language),
            _ => return Err(Error::BadRequest),
        };

        let content_id = match lock.home_repository().insert(home_id).await {
            Ok(id) => Ok(ContentID::try_from(id).unwrap()),
            Err(e) => return Err(Error::Unknown(e.to_string())),
        }?;

        let data = ContentData::try_from(data)
            .map_err(|_| Error::Unknown("Can't parse data to json".to_string()))?;

        

        match lock
            .content_repository()
            .insert(content_id, data, language)
            .await
        {
            Ok(id) => Ok(id),
            Err(e) => return Err(Error::Unknown(e.to_string())),
        }?
    };

    uow.into_inner()
        .commit()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(home_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_the_home_id_otherwise() {
        let uow = crate::uow::home::InMemoryHomeUnitOfWork::new();
        let home_id = Ulid::new().to_string();
        let req = Request {
            home_id: home_id.clone(),
            data: "data".to_string(),
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(id) => assert_eq!(id, home_id),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_bad_request_error_when_request_is_invalid() {
        let uow = crate::uow::home::InMemoryHomeUnitOfWork::new();
        let home_id = Ulid::new().to_string();

        let req = Request {
            home_id,
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
        let uow = crate::uow::home::InMemoryHomeUnitOfWork::new();
        let uow = uow.with_error();
        let home_id = Ulid::new().to_string();

        let req = Request {
            home_id,
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
