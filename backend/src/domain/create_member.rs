use crate::domain::entities::{ContentData, ContentID, Language, MemberData, MemberID};
use crate::repositories::content_repository::IContentRepository;
use crate::repositories::member_repository::IMemberRepository;
use crate::uow::member::IMemberUnitOfWork;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) member_id: String,
    pub(crate) data: Data,
    pub(crate) language: String,
}

pub(crate) struct Data {
    pub(crate) name: String,
    pub(crate) description: String,
}

pub(crate) enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub async fn execute<IUnitOfWork, IMemberRepo, IContentRepo>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<String, Error>
where
    IMemberRepo: IMemberRepository + Send + Sync,
    IContentRepo: IContentRepository + Send + Sync,
    IUnitOfWork: IMemberUnitOfWork<MemberRepo = IMemberRepo, ContentRepo = IContentRepo>,
{
    let mut lock = uow.lock().await;

    let member_id = MemberID::try_from(req.member_id).map_err(|_| Error::BadRequest)?;
    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;
    let data = MemberData::try_from(req.data).map_err(|_| Error::BadRequest)?;

    let content_id = match lock.member_repository().insert(member_id).await {
        Ok(id) => Ok(ContentID(id.0)),
        Err(crate::repositories::member_repository::InsertError::Conflict) => Err(Error::Conflict),
        Err(crate::repositories::member_repository::InsertError::Unknown) => Err(Error::Unknown),
    }?;

    let data = ContentData::try_from(data).map_err(|_| Error::BadRequest)?;

    let content_id = match lock
        .content_repository()
        .insert(content_id, data, language)
        .await
    {
        Ok(id) => Ok(id.0),
        Err(crate::repositories::content_repository::InsertError::Conflict) => Err(Error::Conflict),
        Err(crate::repositories::content_repository::InsertError::Unknown) => Err(Error::Unknown),
    }?;

    drop(lock);
    uow.into_inner()
        .commit()
        .await
        .map_err(|_| Error::Unknown)?;

    Ok(content_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::member_repository::IMemberRepository;
    use crate::uow::member::IMemberUnitOfWork;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_the_member_id_otherwise() {
        let uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id: member_id.clone(),
            data: Data {
                name: "Boris".to_string(),
                description: "Boris is an engineer".to_string(),
            },
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(id) => assert_eq!(id, member_id),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_bad_request_error_when_request_is_invalid() {
        let uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        let member_id = Ulid::new().to_string();

        let req = Request {
            member_id,
            data: Data {
                name: "".to_string(),
                description: "Boris is an engineer".to_string(),
            },
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_a_conflict_error_when_member_id_is_already_exists() {
        let member_id = MemberID::try_from(Ulid::new().to_string()).unwrap();
        let duplicated_member_id = member_id.0.clone();
        let mut uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        uow.member_repository()
            .insert(member_id)
            .await
            .expect("Failed to insert a fake data");

        let req = Request {
            member_id: duplicated_member_id,
            data: Data {
                name: "Boris".to_string(),
                description: "Boris is an engineer".to_string(),
            },
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        let uow = uow.with_error();
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id,
            data: Data {
                name: "Boris".to_string(),
                description: "Boris is an engineer".to_string(),
            },
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }
}
