#[cfg(test)]
use crate::domain::home::entities::HomeID;
#[cfg(test)]
use crate::domain::member::entities::{
    AvatarData, AvatarJson, ContentData, ContentID, Language, MemberData, MemberID,
};
#[cfg(test)]
use crate::domain::service::entities::ServiceID;
#[cfg(test)]
use crate::repositories::avatar_repository::IAvatarRepository;
#[cfg(test)]
use crate::repositories::content_repository::IContentRepository;
#[cfg(test)]
use crate::repositories::home_repository::IHomeRepository;
#[cfg(test)]
use crate::repositories::member_repository::IMemberRepository;
#[cfg(test)]
use crate::repositories::service_repository::IServiceRepository;
#[cfg(test)]
use crate::uow::home::IHomeUnitOfWork;
#[cfg(test)]
use crate::uow::home::InMemoryHomeUnitOfWork;
#[cfg(test)]
use crate::uow::member::{IMemberUnitOfWork, InMemoryMemberUnitOfWork};
#[cfg(test)]
use crate::uow::service::IServiceUnitOfWork;
#[cfg(test)]
use crate::uow::service::InMemoryServiceUnitOfWork;

#[cfg(test)]
pub async fn create_fake_member_helper(
    member_id: MemberID,
    content: Option<MemberData>,
    avatar_data: Option<AvatarData>,
    language: Language,
    error: bool,
) -> InMemoryMemberUnitOfWork {
    let mut uow = InMemoryMemberUnitOfWork::new();
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
        let json = AvatarJson::try_from(avatar_data.unwrap()).expect("Failed to parse avatar json");
        uow.avatar_repository()
            .insert(member_id.clone(), json)
            .await
            .expect("Failed to insert a fake avatar");
    } else {
        // TODO: needs to initialize avatar repository
        let _ = uow.avatar_repository();
    }

    if error {
        uow.with_error()
    } else {
        uow
    }
}

#[cfg(test)]
pub async fn create_fake_service_helper(
    service_id: ServiceID,
    content: Option<ContentData>,
    language: Language,
    error: bool,
) -> InMemoryServiceUnitOfWork {
    let mut uow = InMemoryServiceUnitOfWork::new();
    uow.service_repository()
        .insert(service_id.clone())
        .await
        .expect("Failed to insert a fake member");
    if content.is_some() {
        let content_id = ContentID::try_from(service_id.clone()).unwrap();

        uow.content_repository()
            .insert(content_id, content.unwrap(), language)
            .await
            .expect("Failed to insert a fake content");
    } else {
        // TODO: needs to initialize avatar repository
        let _ = uow.content_repository();
    }

    if error {
        uow.with_error()
    } else {
        uow
    }
}

#[cfg(test)]
pub async fn create_fake_home_helper(
    home_id: HomeID,
    content: Option<ContentData>,
    language: Language,
    error: bool,
) -> InMemoryHomeUnitOfWork {
    let mut uow = InMemoryHomeUnitOfWork::new();
    uow.home_repository()
        .insert(home_id.clone())
        .await
        .expect("Failed to insert a fake member");
    if content.is_some() {
        let content_id = ContentID::try_from(home_id.clone()).unwrap();

        uow.content_repository()
            .insert(content_id, content.unwrap(), language)
            .await
            .expect("Failed to insert a fake content");
    } else {
        // TODO: needs to initialize avatar repository
        let _ = uow.content_repository();
    }

    if error {
        uow.with_error()
    } else {
        uow
    }
}
