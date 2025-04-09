use crate::domain::entities::{ResourceID, ResourceType};
use crate::repositories::IResourceRepository;
use crate::uow::IResourceUnitOfWork;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Request {
    pub(crate) id: String,
    pub(crate) resource_type: ResourceType,
}

pub enum Error {
    BadRequest,
    NotFound,
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<(), Error>
where
    IUnitOfWork: IResourceUnitOfWork,
{
    {
        let mut lock = uow.lock().await;

        // parse the given id and language to the specified type for type safety
        let id = match ResourceID::try_from(req.id) {
            Ok(id) => id,
            _ => return Err(Error::BadRequest),
        };

        match lock
            .resource_repository()
            .contains(&id, &req.resource_type)
            .await
        {
            Ok(exist) if exist => {
                // delete the resource from the repository
                lock.resource_repository()
                    .delete(&id, &req.resource_type)
                    .await
                    .map_err(|e| Error::Unknown(e.to_string()))?;
            }
            Ok(_) => return Err(Error::NotFound),
            Err(e) => return Err(Error::Unknown(e.to_string())),
        }
    }

    // commit the transaction
    uow.into_inner()
        .commit()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Resource;
    use crate::domain::resources::test_helpers::tests::{
        create_resources, create_some_fake_data_and_return_uow,
    };
    use ulid::Ulid;

    fn to_resource_type(resource: &Resource) -> ResourceType {
        match resource {
            Resource::Member(_) => ResourceType::Member,
            Resource::Service(_) => ResourceType::Service,
            Resource::Home(_) => ResourceType::Home,
            Resource::Contact(_) => ResourceType::Contact,
            Resource::Article(_) => ResourceType::Article,
            Resource::Category(_) => ResourceType::Category,
        }
    }

    #[tokio::test]
    async fn it_should_delete_a_resource_successfully_otherwise() {
        let resources = create_resources();

        for resource in resources.clone() {
            let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;

            let (id, resource) = r[0].clone();

            let req = Request {
                id: id.to_string(),
                resource_type: to_resource_type(&resource),
            };

            let res = execute(Mutex::new(uow), req).await;
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn it_should_return_a_not_found_error_when_resource_does_not_exist() {
        let resources = create_resources();

        for resource in resources.clone() {
            let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;

            let (_, resource) = r[0].clone();

            let req = Request {
                id: Ulid::new().to_string(),
                resource_type: to_resource_type(&resource),
            };

            let res = execute(Mutex::new(uow), req).await;
            match res {
                Err(Error::NotFound) => {}
                _ => unreachable!(),
            }
        }
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_unexpected_error_has_encountered() {
        let resources = create_resources();

        for resource in resources.clone() {
            let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;

            let (id, resource) = r[0].clone();

            let req = Request {
                id: id.to_string(),
                resource_type: to_resource_type(&resource),
            };

            let res = execute(Mutex::new(uow.with_error()), req).await;
            match res {
                Err(Error::Unknown(_)) => {}
                _ => unreachable!(),
            }
        }
    }
}
