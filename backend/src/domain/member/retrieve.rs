use crate::domain::member::entities::{Language, Member, MemberID};
use crate::uow::member::IMemberUnitOfWork;
use tokio::sync::Mutex;

pub(crate) struct Request {
    pub(crate) member_id: String,
    pub(crate) language: String,
    pub(crate) default_language: Language,
}

#[derive(Debug)]
pub(crate) enum Error {
    BadRequest,
    MemberNotFound,
    Unknown,
}

pub async fn execute<IUnitOfWork>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<Option<Member>, Error>
where
    IUnitOfWork: IMemberUnitOfWork,
{
    let mut lock = uow.lock().await;

    let member_id = MemberID::try_from(req.member_id).map_err(|_| Error::BadRequest)?;
    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let res = lock
        .get_member(&member_id, &language)
        .await
        .map_err(|_| Error::Unknown)?;

    match res {
        None => {
            if language != req.default_language {
                match lock.get_member(&member_id, &req.default_language).await {
                    Ok(Some(res)) => Ok(Some(res)),
                    Ok(None) => Err(Error::MemberNotFound),
                    Err(_) => Err(Error::Unknown),
                }
            } else {
                Err(Error::MemberNotFound)
            }
        }
        Some(res) => Ok(Some(res)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::member::entities::{AvatarData, MemberData};
    use crate::domain::test_helper::create_fake_member_helper;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_return_a_member_otherwise() {
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
            member_id: member_id.as_str().to_string(),
            language: "en".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(Some(member)) => {
                assert_eq!(member.member_id, member_id.as_str());
                let content_data: MemberData = serde_json::from_value(member.content).unwrap();
                assert_eq!(content_data.name, content.name);
                assert_eq!(content_data.description, content.description);
                let avatar_data: AvatarData =
                    serde_json::from_value(member.avatar_data.unwrap()).unwrap();
                assert_eq!(avatar.large_image, avatar_data.large_image);
                assert_eq!(avatar.small_image, avatar_data.small_image);
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_a_default_language_member_otherwise() {
        let member_id = MemberID::try_from(Ulid::new().to_string()).unwrap();
        let data = MemberData {
            name: "Boris".to_string(),
            description: "Boris is a engineer".to_string(),
        };
        let uow = create_fake_member_helper(
            member_id.clone(),
            Some(data.clone()),
            None,
            Language::EN,
            false,
        )
        .await;

        let req = Request {
            member_id: member_id.as_str().to_string(),
            language: "zh".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Ok(Some(member)) => {
                assert_eq!(member.member_id, member_id.as_str());
                let content_data: MemberData = serde_json::from_value(member.content).unwrap();
                assert_eq!(content_data.name, data.name);
                assert_eq!(content_data.description, data.description);
            }
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_member_is_not_found() {
        let member_id = MemberID::try_from(Ulid::new().to_string()).unwrap();
        let uow =
            create_fake_member_helper(member_id.clone(), None, None, Language::EN, false).await;

        let req = Request {
            member_id: "not_found_id".to_string(),
            language: "en".to_string(),
            default_language: Language::EN,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::MemberNotFound) => {}
            _ => unreachable!(),
        }
    }
}
