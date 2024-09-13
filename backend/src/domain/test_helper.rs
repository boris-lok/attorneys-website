#[cfg(test)]
use crate::domain::entities::{
    AvatarData, AvatarJson, ContentData, ContentID, Language, MemberData, MemberID,
};
#[cfg(test)]
use crate::repositories::avatar_repository::IAvatarRepository;
#[cfg(test)]
use crate::repositories::content_repository::IContentRepository;
#[cfg(test)]
use crate::repositories::member_repository::IMemberRepository;
#[cfg(test)]
use crate::uow::member::{IMemberUnitOfWork, InMemoryMemberUnitOfWork};

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
