use crate::domain::entities::{Language, SimpleMember};
use crate::domain::member::entities::{Language, SimpleMember};
use crate::uow::member::IMemberUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    language: String,
}

pub(crate) enum Error {
    BadRequest,
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<Vec<SimpleMember>, Error>
where
    IUnitOfWork: IMemberUnitOfWork,
{
    let mut lock = uow.lock().await;

    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let res = lock.get_all_members_by_language(&language).await;

    match res {
        Ok(members) => Ok(members),
        Err(e) => Err(Error::Unknown(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{AvatarData, MemberData, MemberID};
    use crate::domain::test_helper::create_fake_member_helper;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_an_empty_list_otherwise() {
        let mut uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        let _ = uow.content_repository();

        let req = Request {
            language: "en".to_string(),
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
    async fn it_should_return_a_members_list_otherwise() {
        let member_id = MemberID::try_from(Ulid::new().to_string()).unwrap();
        let content = MemberData {
            name: "Boris".to_string(),
            description: "Boris is a engineer".to_string(),
        };
        let avatar = AvatarData {
            large_image: "large".to_string(),
            small_image: "small".to_string(),
        };
        let uow = create_fake_member_helper(
            member_id.clone(),
            Some(content.clone()),
            Some(avatar.clone()),
            Language::EN,
            false,
        )
        .await;

        let req = Request {
            language: "en".to_string(),
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
