use crate::domain::resources::entity::{
    ContentData, ContentID, Language, ResourceID, ResourceType, UpdateResourceRequest,
};
use crate::domain::services::resource::ResourceUoW;
use crate::domain::uow::common::UnitOfWorkFactory;

pub struct Request {
    pub id: ResourceID,
    pub kind: ResourceType,
    pub data: ContentData,
    pub seq: i32,
    pub language: Language,
}

pub enum Error {
    Unknown(String),
}

pub async fn execute<F: UnitOfWorkFactory>(
    service: &ResourceUoW<F>,
    req: Request,
) -> Result<ContentID, Error> {
    let id = ContentID::from(req.id.clone());

    service
        .update(UpdateResourceRequest {
            id: req.id,
            kind: req.kind,
            data: req.data,
            seq: req.seq,
            language: req.language,
        })
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(id)
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::entity::{
//         ArticleData, CategoryData, ContactData, HomeData, MemberData, ServiceData,
//     };
//     use crate::domain::resources::test_helpers::tests::{
//         create_resources, create_some_fake_data_and_return_uow,
//     };
//     use serde_json::json;
//     use ulid::Ulid;
//
//     fn update_resource(resource: Resource) -> Resource {
//         match resource {
//             Resource::Member(m) => Resource::Member(MemberData {
//                 name: "new name".to_string(),
//                 ..m
//             }),
//             Resource::Service(s) => Resource::Service(ServiceData {
//                 title: "new title".to_string(),
//                 ..s
//             }),
//             Resource::Home(_) => Resource::Home(HomeData {
//                 data: "new data".to_string(),
//             }),
//             Resource::Contact(_) => Resource::Contact(ContactData {
//                 data: json!({
//                     "address": "new address",
//                 }),
//             }),
//             Resource::Article(a) => Resource::Article(ArticleData {
//                 title: "new title".to_string(),
//                 ..a
//             }),
//             Resource::Category(_) => {
//                 Resource::Category(CategoryData::new(None, "new category".to_string()))
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn it_should_update_successfully_otherwise() {
//         let resources = create_resources();
//
//         for resource in resources.clone() {
//             let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;
//
//             let (id, resource) = r[0].clone();
//
//             let updated_resource = update_resource(resource);
//
//             let req = Request {
//                 id: id.to_string().clone(),
//                 data: updated_resource.clone(),
//                 language: "zh".to_string(),
//                 seq: 0,
//             };
//
//             let res = execute(Mutex::new(uow), req).await;
//             // TODO: check the updated data
//             assert!(res.is_ok());
//         }
//     }
//
//     #[tokio::test]
//     async fn it_should_return_a_not_found_error_when_resource_does_not_exist() {
//         let resources = create_resources();
//
//         for resource in resources.clone() {
//             let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;
//
//             let (_, resource) = r[0].clone();
//
//             let updated_resource = update_resource(resource);
//
//             let req = Request {
//                 id: Ulid::new().to_string(),
//                 data: updated_resource,
//                 language: "zh".to_string(),
//                 seq: 0,
//             };
//
//             let res = execute(Mutex::new(uow), req).await;
//             match res {
//                 Err(Error::NotFound) => {}
//                 _ => unreachable!(),
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn it_should_return_an_unknown_error_when_unexpected_error_has_encountered() {
//         let resources = create_resources();
//
//         for resource in resources.clone() {
//             let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;
//
//             let (id, resource) = r[0].clone();
//
//             let updated_resource = update_resource(resource);
//
//             let req = Request {
//                 id: id.to_string().clone(),
//                 data: updated_resource.clone(),
//                 language: "zh".to_string(),
//                 seq: 0,
//             };
//
//             let res = execute(Mutex::new(uow.with_error()), req).await;
//             match res {
//                 Err(Error::Unknown(_)) => {}
//                 _ => unreachable!(),
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn it_should_create_a_new_content_for_different_language() {
//         let resources = create_resources();
//         let resource = resources.first().unwrap().clone();
//
//         let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;
//
//         let (id, resource) = r[0].clone();
//
//         let req = Request {
//             id: id.clone().to_string(),
//             data: resource,
//             language: "en".to_string(),
//             seq: 0,
//         };
//
//         let uow = Mutex::new(uow);
//
//         let res = execute(uow, req).await;
//         // TODO: check the updated data
//         assert!(res.is_ok());
//     }
// }
