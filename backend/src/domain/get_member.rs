use crate::domain::entities::{Language, Member, MemberID};
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
    use crate::domain::entities::{AvatarData, AvatarJson, ContentData, ContentID, MemberData};
    use crate::repositories::avatar_repository::IAvatarRepository;
    use crate::repositories::content_repository::IContentRepository;
    use crate::repositories::member_repository::IMemberRepository;
    use crate::uow::member::InMemoryMemberUnitOfWork;
    use ulid::Ulid;

    async fn create_fake_member_helper(
        member_id: MemberID,
        content: Option<MemberData>,
        avatar_data: Option<AvatarData>,
        language: Language,
    ) -> InMemoryMemberUnitOfWork {
        let mut uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        uow.member_repository()
            .insert(member_id.clone())
            .await
            .expect("Failed to insert a fake member");
        if content.is_some() {
            let content_id = ContentID::try_from(member_id.clone()).unwrap();
            uow.content_repository()
                .insert(
                    content_id,
                    ContentData::try_from(content.unwrap()).unwrap(),
                    language,
                )
                .await
                .expect("Failed to insert a fake content");
        } else {
            // TODO: needs to initialize avatar repository
            let _ = uow.content_repository();
        }

        if avatar_data.is_some() {
            let json =
                AvatarJson::try_from(avatar_data.unwrap()).expect("Failed to parse avatar json");
            uow.avatar_repository()
                .insert(member_id.clone(), json)
                .await
                .expect("Failed to insert a fake avatar");
        } else {
            // TODO: needs to initialize avatar repository
            let _ = uow.avatar_repository();
        }

        uow
    }

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
        let uow =
            create_fake_member_helper(member_id.clone(), Some(data.clone()), None, Language::EN)
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
        let uow = create_fake_member_helper(member_id.clone(), None, None, Language::EN).await;

        let req = Request {
            member_id: member_id.as_str().to_string(),
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
