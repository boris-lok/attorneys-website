use crate::domain::entities::{Language, MemberID, MemberName};

struct Request {
    member_id: String,
    name: String,
    default_language: String,
}

enum Response {
    BadRequest,
    Ok(String),
}

fn execute(req: Request) -> Response {
    match (
        MemberID::try_from(req.member_id),
        MemberName::try_from(req.name),
        Language::try_from(req.default_language),
    ) {
        (Ok(id), Ok(name), Ok(lang)) => Response::Ok(id.0),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ulid::Ulid;

    #[test]
    fn it_should_return_the_member_id_otherwise() {
        let member_id = Ulid::new().to_string();
        let req = Request {
            member_id: member_id.clone(),
            name: String::from("Boris"),
            default_language: "en".to_string(),
        };

        let res = execute(req);

        match res {
            Response::Ok(id) => assert_eq!(id, member_id),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_bad_request_error_when_request_is_invalid() {
        let member_id = Ulid::new().to_string();

        let req = Request {
            member_id,
            name: String::from(""),
            default_language: "en".to_string(),
        };

        let res = execute(req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }
}
