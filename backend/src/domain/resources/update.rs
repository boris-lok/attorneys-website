use crate::domain::entities::{ContentID, Language, Resource, ResourceID};
use crate::repositories::IContentRepository;
use crate::repositories::IResourceRepository;
use crate::uow::IResourceUnitOfWork;
use tokio::sync::Mutex;

pub struct Request {
    pub id: String,
    pub data: Resource,
    pub language: String,
    pub seq: i32,
}

pub enum Error {
    BadRequest,
    NotFound,
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<ContentID, Error>
where
    IUnitOfWork: IResourceUnitOfWork,
{
    let id = {
        let mut lock = uow.lock().await;

        let (kind, data) = req
            .data
            .into_typed_content()
            .map_err(|_| Error::BadRequest)?;

        let id = ResourceID::try_from(req.id).map_err(|_| Error::BadRequest)?;
        let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

        if !lock
            .resource_repository()
            .contains(&id, &kind)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?
        {
            return Err(Error::NotFound);
        }

        match lock.resource_repository().update_seq(&id, req.seq).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Unknown(e.to_string())),
        }

        let id = ContentID::from(id);

        if !lock
            .content_repository()
            .contains(&id, &language)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?
        {
            // insert the content into the content repository and retrieve the content id
            match lock
                .content_repository()
                .insert(id.clone(), data, language)
                .await
            {
                Ok(_) => Ok(id),
                Err(e) => return Err(Error::Unknown(e.to_string())),
            }
        } else {
            // insert the content into the content repository and retrieve the content id
            match lock.content_repository().update(&id, data, language).await {
                Ok(_) => Ok(id),
                Err(e) => return Err(Error::Unknown(e.to_string())),
            }
        }
    }?;

    // commit the transaction
    uow.into_inner()
        .commit()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        ArticleData, CategoryData, ContactData, HomeData, MemberData, ServiceData,
    };
    use crate::domain::resources::test_helpers::tests::{
        create_resources, create_some_fake_data_and_return_uow,
    };
    use serde_json::json;
    use ulid::Ulid;

    fn update_resource(resource: Resource) -> Resource {
        match resource {
            Resource::Member(m) => Resource::Member(MemberData {
                name: "new name".to_string(),
                ..m
            }),
            Resource::Service(s) => Resource::Service(ServiceData {
                title: "new title".to_string(),
                ..s
            }),
            Resource::Home(_) => Resource::Home(HomeData {
                data: "new data".to_string(),
            }),
            Resource::Contact(_) => Resource::Contact(ContactData {
                data: json!({
                    "address": "new address",
                }),
            }),
            Resource::Article(a) => Resource::Article(ArticleData {
                title: "new title".to_string(),
                ..a
            }),
            Resource::Category(_) => {
                Resource::Category(CategoryData::new(None, "new category".to_string()))
            }
        }
    }

    #[tokio::test]
    async fn it_should_update_successfully_otherwise() {
        let resources = create_resources();

        for resource in resources.clone() {
            let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;

            let (id, resource) = r[0].clone();

            let updated_resource = update_resource(resource);

            let req = Request {
                id: id.to_string().clone(),
                data: updated_resource.clone(),
                language: "zh".to_string(),
                seq: 0,
            };

            let res = execute(Mutex::new(uow), req).await;
            // TODO: check the updated data
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn it_should_return_a_not_found_error_when_resource_does_not_exist() {
        let resources = create_resources();

        for resource in resources.clone() {
            let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;

            let (_, resource) = r[0].clone();

            let updated_resource = update_resource(resource);

            let req = Request {
                id: Ulid::new().to_string(),
                data: updated_resource,
                language: "zh".to_string(),
                seq: 0,
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

            let updated_resource = update_resource(resource);

            let req = Request {
                id: id.to_string().clone(),
                data: updated_resource.clone(),
                language: "zh".to_string(),
                seq: 0,
            };

            let res = execute(Mutex::new(uow.with_error()), req).await;
            match res {
                Err(Error::Unknown(_)) => {}
                _ => unreachable!(),
            }
        }
    }

    #[tokio::test]
    async fn it_should_create_a_new_content_for_different_language() {
        let resources = create_resources();
        let resource = resources.first().unwrap().clone();

        let (uow, r) = create_some_fake_data_and_return_uow(vec![resource]).await;

        let (id, resource) = r[0].clone();

        let req = Request {
            id: id.clone().to_string(),
            data: resource,
            language: "en".to_string(),
            seq: 0,
        };

        let uow = Mutex::new(uow);

        let res = execute(uow, req).await;
        // TODO: check the updated data
        assert!(res.is_ok());
    }
}
