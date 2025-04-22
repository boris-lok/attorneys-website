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
    Unknown(String),
}

pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<ContentID, Error>
where
    IUnitOfWork: IResourceUnitOfWork,
{
    let id = {
        let mut lock = uow.lock().await;

        // extract the resource and convert it to a resource type
        let (kind, data) = req
            .data
            .into_typed_content()
            .map_err(|_| Error::BadRequest)?;

        // parse the given id and language to the specified type for type safety
        let (id, language) = match (
            ResourceID::try_from(req.id),
            Language::try_from(req.language),
        ) {
            (Ok(id), Ok(language)) => (id, language),
            _ => return Err(Error::BadRequest),
        };

        // insert the resource into the resource repository and retrieve the content id
        let content_id = match lock.resource_repository().insert(id, kind, req.seq).await {
            Ok(id) => ContentID::from(id),
            Err(e) => return Err(Error::Unknown(e.to_string())),
        };

        // insert the content into the content repository and retrieve the content id
        match lock
            .content_repository()
            .insert(content_id, data, language)
            .await
        {
            Ok(id) => id,
            Err(e) => return Err(Error::Unknown(e.to_string())),
        }
    };

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
    use crate::domain::entities::{ArticleData, ContactData, HomeData, MemberData, ServiceData};
    use crate::uow::InMemory;
    use serde_json::json;
    use ulid::Ulid;

    #[tokio::test]
    async fn it_should_create_a_resource_successful_otherwise() {
        let member_data = MemberData::new("boris".to_string(), "description".to_string());
        let service_data =
            ServiceData::new("title".to_string(), "data".to_string(), "icon".to_string());
        let home_data = HomeData::new("data".to_string());
        let contact = json!({
            "address": "address".to_string(),
            "phone": "1234".to_string(),
            "email": "info@example.com".to_string(),
        });
        let contact_data = ContactData::new(contact);
        let article_data = ArticleData::new(None, "title".to_string(), "data".to_string());

        let different_data = vec![
            Resource::Member(member_data),
            Resource::Service(service_data),
            Resource::Home(home_data),
            Resource::Contact(contact_data),
            Resource::Article(article_data),
        ];

        for d in different_data {
            let uow = InMemory::new();
            let id = Ulid::new().to_string();

            let req = Request {
                id: id.clone(),
                data: d,
                language: "zh".to_string(),
                seq: 0,
            };

            let res = execute(Mutex::new(uow), req).await;

            match res {
                Ok(id) => {
                    assert_eq!(id.as_str(), id.as_str())
                }
                Err(_) => unreachable!(),
            }
        }
    }

    #[tokio::test]
    async fn it_should_return_bad_request_when_data_is_missing_or_invalid() {
        let missing_or_invalid_data = vec![
            // name is missing
            Resource::Member(MemberData::new("".to_string(), "description".to_string())),
            // description is missing
            Resource::Member(MemberData::new("boris".to_string(), "".to_string())),
            // The name is conducted by spaces
            Resource::Member(MemberData::new("  ".to_string(), "description".to_string())),
            // The description is conducted by spaces
            Resource::Member(MemberData::new("boris".to_string(), "  ".to_string())),
            // title is missing
            Resource::Service(ServiceData::new(
                "".to_string(),
                "data".to_string(),
                "icon".to_string(),
            )),
            // data is missing
            Resource::Service(ServiceData::new(
                "title".to_string(),
                "".to_string(),
                "icon".to_string(),
            )),
            // The title is conducted by spaces
            Resource::Service(ServiceData::new(
                "  ".to_string(),
                "data".to_string(),
                "icon".to_string(),
            )),
            // The data is conducted by spaces
            Resource::Service(ServiceData::new(
                "title".to_string(),
                " ".to_string(),
                "icon".to_string(),
            )),
            // data is missing
            Resource::Home(HomeData::new("".to_string())),
            // data is conducted by spaces
            Resource::Home(HomeData::new(" ".to_string())),
            // title is missing
            Resource::Article(ArticleData::new(None, "".to_string(), "data".to_string())),
            // data is missing
            Resource::Article(ArticleData::new(None, "title".to_string(), "".to_string())),
            // The title is conducted by spaces
            Resource::Article(ArticleData::new(None, "  ".to_string(), "data".to_string())),
            // The data is conducted by spaces
            Resource::Article(ArticleData::new(None, "title".to_string(), " ".to_string())),
        ];

        for d in missing_or_invalid_data {
            let uow = InMemory::new();
            let id = Ulid::new().to_string();

            let req = Request {
                id: id.clone(),
                data: d,
                language: "zh".to_string(),
                seq: 0,
            };

            let res = execute(Mutex::new(uow), req).await;

            match res {
                Err(Error::BadRequest) => {}
                _ => unreachable!(),
            }
        }
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_unexpected_error_is_encountered() {
        let data = Resource::Member(MemberData::new(
            "boris".to_string(),
            "description".to_string(),
        ));
        let uow = InMemory::new().with_error();
        let id = Ulid::new().to_string();

        let req = Request {
            id: id.clone(),
            data,
            language: "zh".to_string(),
            seq: 0,
        };

        let res = execute(Mutex::new(uow), req).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        }
    }
}
