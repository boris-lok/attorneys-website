use crate::domain::member::entities::MemberID;
use crate::repositories::member_repository::IMemberRepository;
use crate::uow::member::IMemberUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) member_id: String,
}

pub(crate) enum Error {
    BadRequest,
    MemberNotFound,
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<(), Error>
where
    IUnitOfWork: IMemberUnitOfWork,
{
    let mut lock = uow.lock().await;

    let member_id = MemberID::try_from(req.member_id).map_err(|_| Error::BadRequest)?;

    let affected = match lock.member_repository().delete(member_id).await {
        Ok(affected) => affected,
        Err(e) => return Err(Error::Unknown(e.to_string())),
    };

    drop(lock);
    uow.into_inner()
        .commit()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    match affected {
        true => Ok(()),
        false => Err(Error::MemberNotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::member::entities::{AvatarData, Language, MemberData, MemberID};
    use crate::domain::test_helper::create_fake_member_helper;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_success_otherwise() {
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
            member_id: member_id.to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(_) => {}
            Err(_) => unreachable!(),
        }
    }
    #[tokio::test]
    async fn it_should_return_an_error_when_member_does_not_exist() {
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

        let not_exists_id = Ulid::new().to_string();

        let req = Request {
            member_id: not_exists_id,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::MemberNotFound) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_unexpected_error_encountered() {
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
            true,
        )
        .await;

        let req = Request {
            member_id: member_id.to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        }
    }
}
