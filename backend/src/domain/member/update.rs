use crate::domain::member::entities::{
    ContentData, ContentID, Data, Language, MemberData, MemberID,
};
use crate::repositories::content_repository::IContentRepository;
use crate::repositories::member_repository::IMemberRepository;
use crate::uow::member::IMemberUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) member_id: String,
    pub(crate) data: Data,
    pub(crate) language: String,
}

#[derive(Debug)]
pub(crate) enum Error {
    BadRequest,
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<(), Error>
where
    IUnitOfWork: IMemberUnitOfWork,
{
    let mut lock = uow.lock().await;

    let (member_id, data, language) = match (
        MemberID::try_from(req.member_id),
        MemberData::try_from(req.data),
        Language::try_from(req.language),
    ) {
        (Ok(member_id), Ok(data), Ok(language)) => (member_id, data, language),
        _ => return Err(Error::BadRequest),
    };

    match lock.member_repository().contains(&member_id).await {
        Ok(exist) if !exist => return Err(Error::BadRequest),
        Err(e) => return Err(Error::Unknown(e.to_string())),
        Ok(_) => {}
    };

    let content_id = ContentID::try_from(member_id)
        .map_err(|_| Error::Unknown("Can't parse member_id to content_id".to_string()))?;
    let data = ContentData::try_from(data)
        .map_err(|_| Error::Unknown("Can't parse data to json".to_string()))?;

    match lock
        .content_repository()
        .update(&content_id, data, language)
        .await
    {
        Ok(_) => {}
        Err(e) => return Err(Error::Unknown(e.to_string())),
    };

    drop(lock);
    uow.into_inner()
        .commit()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::member::entities::AvatarData;
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

        let new_data = Data {
            name: "Boris updated".to_string(),
            description: "Boris is an engineer updated".to_string(),
        };

        let req = Request {
            member_id: member_id.to_string(),
            data: new_data,
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(_) => {
                // TODO: check the data is updated
            }
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

        let new_data = Data {
            name: "Boris updated".to_string(),
            description: "Boris is an engineer updated".to_string(),
        };

        let not_exists_id = Ulid::new().to_string();

        let req = Request {
            member_id: not_exists_id,
            data: new_data,
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::BadRequest) => {}
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

        let new_data = Data {
            name: "Boris updated".to_string(),
            description: "Boris is an engineer updated".to_string(),
        };

        let req = Request {
            member_id: member_id.to_string(),
            data: new_data,
            language: "en".to_string(),
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        }
    }
}
