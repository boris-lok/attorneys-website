use crate::domain::resources::entity::ResourceID;
use crate::domain::uow::common::UnitOfWorkFactory;
use crate::domain::uow::resource::ResourceUoW;

pub enum Error {
    Unknown(String),
}

pub async fn execute<F: UnitOfWorkFactory>(
    service: &ResourceUoW<F>,
    id: &ResourceID,
) -> Result<(), Error> {
    {
        service
            .delete(id)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        Ok(())
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::entities::Resource;
//     use crate::domain::resources::test_helpers::tests::{
//         create_resources, create_some_fake_data_and_return_uow,
//     };
//     use ulid::Ulid;
//
//     fn to_resource_type(resource: &Resource) -> ResourceType {
//         match resource {
//             Resource::Member(_) => ResourceType::Member,
//             Resource::Service(_) => ResourceType::Service,
//             Resource::Home(_) => ResourceType::Home,
//             Resource::Contact(_) => ResourceType::Contact,
//             Resource::Article(_) => ResourceType::Article,
//             Resource::Category(_) => ResourceType::Category,
//         }
//     }
//
//     #[tokio::test]
//     async fn it_should_delete_a_resource_successfully_otherwise() {
//         let resources = create_resources();
//
//         for resource in resources.clone() {
//             let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;
//
//             let (id, resource) = r[0].clone();
//
//             let req = Request {
//                 id: id.to_string(),
//                 resource_type: to_resource_type(&resource),
//             };
//
//             let res = execute(Mutex::new(uow), req).await;
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
//             let req = Request {
//                 id: Ulid::new().to_string(),
//                 resource_type: to_resource_type(&resource),
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
//             let req = Request {
//                 id: id.to_string(),
//                 resource_type: to_resource_type(&resource),
//             };
//
//             let res = execute(Mutex::new(uow.with_error()), req).await;
//             match res {
//                 Err(Error::Unknown(_)) => {}
//                 _ => unreachable!(),
//             }
//         }
//     }
// }
