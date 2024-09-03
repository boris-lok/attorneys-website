use crate::domain::entities::{Language, MemberID, MemberName};
use crate::repositories::member_repository::{InsertError, MemberRepository};
use std::sync::Arc;

pub(crate) struct Request {
    pub(crate) member_id: String,
    pub(crate) name: String,
    pub(crate) language: String,
}

pub(crate) enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub fn execute(repo: Arc<dyn MemberRepository>, req: Request) -> Result<String, Error> {
    match (
        MemberID::try_from(req.member_id),
        MemberName::try_from(req.name),
        Language::try_from(req.language),
    ) {
        (Ok(id), Ok(name), Ok(lang)) => match repo.insert(id, name, lang) {
            Ok(member_id) => Ok(member_id.0),
            Err(InsertError::Conflict) => Err(Error::Conflict),
            Err(InsertError::Unknown) => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::member_repository::InMemoryMemberRepository;
    use ulid::Ulid;

    #[test]
    fn it_should_return_the_member_id_otherwise() {
        let repo = Arc::new(InMemoryMemberRepository::new());
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id: member_id.clone(),
            name: String::from("Boris"),
            language: "en".to_string(),
        };

        let res = execute(repo, req);

        match res {
            Ok(id) => assert_eq!(id, member_id),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryMemberRepository::new());
        let member_id = Ulid::new().to_string();

        let req = Request {
            member_id,
            name: String::from(""),
            language: "en".to_string(),
        };

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_member_id_is_already_exists() {
        let member_id = MemberID::try_from(Ulid::new().to_string()).unwrap();
        let name = MemberName::try_from(String::from("Boris")).unwrap();
        let default_language = Language::EN;
        let repo = Arc::new(InMemoryMemberRepository::new());
        let duplicated_member_id = member_id.0.clone();
        repo.insert(member_id, name, default_language)
            .expect("Failed to insert a fake data");

        let req = Request {
            member_id: duplicated_member_id,
            name: "Lily".to_string(),
            language: "en".to_string(),
        };

        let res = execute(repo, req);

        match res {
            Err(Error::Conflict) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryMemberRepository::new().with_error());
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id,
            name: String::from("Boris"),
            language: "en".to_string(),
        };

        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }
}
