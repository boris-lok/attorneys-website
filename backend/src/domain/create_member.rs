use crate::domain::entities::{Language, MemberID, MemberName};
use crate::repositories::member_repository::{Insert, MemberRepository};

struct Request {
    member_id: String,
    name: String,
    language: String,
}

enum Response {
    BadRequest,
    Conflict,
    Error,
    Ok(String),
}

fn execute(repo: &mut dyn MemberRepository, req: Request) -> Response {
    match (
        MemberID::try_from(req.member_id),
        MemberName::try_from(req.name),
        Language::try_from(req.language),
    ) {
        (Ok(id), Ok(name), Ok(lang)) => match repo.insert(id, name, lang) {
            Insert::Ok(member_id) => Response::Ok(member_id.0),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::member_repository::InMemoryMemberRepository;
    use ulid::Ulid;

    #[test]
    fn it_should_return_the_member_id_otherwise() {
        let mut repo = InMemoryMemberRepository::new();
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id: member_id.clone(),
            name: String::from("Boris"),
            language: "en".to_string(),
        };

        let res = execute(&mut repo, req);

        match res {
            Response::Ok(id) => assert_eq!(id, member_id),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_bad_request_error_when_request_is_invalid() {
        let mut repo = InMemoryMemberRepository::new();
        let member_id = Ulid::new().to_string();

        let req = Request {
            member_id,
            name: String::from(""),
            language: "en".to_string(),
        };

        let res = execute(&mut repo, req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_member_id_is_already_exists() {
        let member_id = MemberID::try_from(Ulid::new().to_string()).unwrap();
        let name = MemberName::try_from(String::from("Boris")).unwrap();
        let default_language = Language::EN;
        let mut repo = InMemoryMemberRepository::new();
        let duplicated_member_id = member_id.0.clone();
        repo.insert(member_id, name, default_language);

        let req = Request {
            member_id: duplicated_member_id,
            name: "Lily".to_string(),
            language: "en".to_string(),
        };

        let res = execute(&mut repo, req);

        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let mut repo = InMemoryMemberRepository::new().with_error();
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id,
            name: String::from("Boris"),
            language: "en".to_string(),
        };

        let res = execute(&mut repo, req);

        match res {
            Response::Error => {}
            _ => unreachable!(),
        }
    }
}
