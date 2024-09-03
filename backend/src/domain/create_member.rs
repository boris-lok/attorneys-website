use crate::domain::entities::{ContentData, ContentID, Language, MemberData, MemberID};
use std::sync::Arc;

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

pub fn execute(uow: Arc<crate::uow::member_uow::MemberUOW>, req: Request) -> Result<String, Error> {
    let member_id = MemberID::try_from(req.member_id).map_err(|_| Error::BadRequest)?;
    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;
    let data = MemberData::try_from(req.data).map_err(|_| Error::BadRequest)?;

    let content_id = match uow.member_repository.insert(member_id) {
        Ok(id) => Ok(ContentID(id.0)),
        Err(crate::repositories::member_repository::InsertError::Conflict) => Err(Error::Conflict),
        Err(crate::repositories::member_repository::InsertError::Unknown) => Err(Error::Unknown),
    }?;

    let data = ContentData::try_from(data).map_err(|_| Error::BadRequest)?;

    match uow.content_repository.insert(content_id, data, language) {
        Ok(id) => Ok(id.0),
        Err(crate::repositories::content_repository::InsertError::Conflict) => Err(Error::Conflict),
        Err(crate::repositories::content_repository::InsertError::Unknown) => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::content_repository::InMemoryContentRepository;
    use crate::repositories::member_repository::{IMemberRepository, InMemoryMemberRepository};
    use ulid::Ulid;

    #[test]
    fn it_should_return_the_member_id_otherwise() {
        let member_repo = Arc::new(InMemoryMemberRepository::new());
        let content_repo = Arc::new(InMemoryContentRepository::new());
        let uow = Arc::new(crate::uow::member_uow::MemberUOW {
            member_repository: member_repo.clone(),
            content_repository: content_repo.clone(),
        });
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id: member_id.clone(),
            data: Data {
                name: "Boris".to_string(),
                description: "Boris is an engineer".to_string(),
            },
            language: "en".to_string(),
        };

        let res = execute(uow, req);

        match res {
            Ok(id) => assert_eq!(id, member_id),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_bad_request_error_when_request_is_invalid() {
        let member_repo = Arc::new(InMemoryMemberRepository::new());
        let content_repo = Arc::new(InMemoryContentRepository::new());
        let uow = Arc::new(crate::uow::member_uow::MemberUOW {
            member_repository: member_repo.clone(),
            content_repository: content_repo.clone(),
        });
        let member_id = Ulid::new().to_string();

        let req = Request {
            member_id,
            data: Data {
                name: "".to_string(),
                description: "Boris is an engineer".to_string(),
            },
            language: "en".to_string(),
        };

        let res = execute(uow, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_member_id_is_already_exists() {
        let member_id = MemberID::try_from(Ulid::new().to_string()).unwrap();
        let member_repo = Arc::new(InMemoryMemberRepository::new());
        let duplicated_member_id = member_id.0.clone();
        member_repo
            .insert(member_id)
            .expect("Failed to insert a fake data");
        let content_repo = Arc::new(InMemoryContentRepository::new());
        let uow = Arc::new(crate::uow::member_uow::MemberUOW {
            member_repository: member_repo.clone(),
            content_repository: content_repo.clone(),
        });

        let req = Request {
            member_id: duplicated_member_id,
            data: Data {
                name: "Boris".to_string(),
                description: "Boris is an engineer".to_string(),
            },
            language: "en".to_string(),
        };

        let res = execute(uow, req);

        match res {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let member_repo = Arc::new(InMemoryMemberRepository::new().with_error());
        let content_repo = Arc::new(InMemoryContentRepository::new());
        let uow = Arc::new(crate::uow::member_uow::MemberUOW {
            member_repository: member_repo.clone(),
            content_repository: content_repo.clone(),
        });
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id,
            data: Data {
                name: "Boris".to_string(),
                description: "Boris is an engineer".to_string(),
            },
            language: "en".to_string(),
        };

        let res = execute(uow, req);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }
}
