#[cfg(test)]
pub(crate) mod tests {
    use crate::domain::entities::{
        ArticleData, CategoryData, ContactData, ContentData, ContentID, HomeData, Language,
        MemberData, Resource, ResourceID, ResourceType, ServiceData,
    };
    use crate::repositories::IContentRepository;
    use crate::repositories::IResourceRepository;
    use crate::uow::{IResourceUnitOfWork, InMemory};
    use serde_json::json;

    pub fn create_resources() -> Vec<Resource> {
        let member = MemberData::new("boris".to_string(), "description".to_string());
        let service = ServiceData::new("title".to_string(), "data".to_string(), "icon".to_string());
        let home = HomeData::new("home".to_string());
        let contact = json!({
            "address": "address".to_string(),
            "phone": "1234".to_string(),
            "email": "info@example.com".to_string(),
        });
        let contact = ContactData::new(contact);
        let article = ArticleData::new(None, "title".to_string(), "data".to_string());
        let category = CategoryData::new(None, "category".to_string());

        vec![
            Resource::Article(article),
            Resource::Service(service),
            Resource::Home(home),
            Resource::Member(member),
            Resource::Contact(contact),
            Resource::Category(category),
        ]
    }
    pub async fn create_some_fake_data_and_return_uow(
        resources: Vec<Resource>,
    ) -> (InMemory, Vec<(ContentID, Resource)>) {
        let mut uow = InMemory::new();
        let mut resources_with_id = vec![];

        for resource in resources {
            let id = ulid::Ulid::new().to_string();
            let resource_id = ResourceID::try_from(id.clone()).unwrap();
            let content_id = ContentID::from(resource_id.clone());
            let content_data = ContentData::try_from(resource.clone()).unwrap();

            match &resource {
                Resource::Member(_) => {
                    uow.resource_repository()
                        .insert(resource_id.clone(), ResourceType::Member, 0)
                        .await
                        .unwrap();
                }
                Resource::Service(_) => {
                    uow.resource_repository()
                        .insert(resource_id.clone(), ResourceType::Service, 1)
                        .await
                        .unwrap();
                }
                Resource::Home(_) => {
                    uow.resource_repository()
                        .insert(resource_id.clone(), ResourceType::Home, 2)
                        .await
                        .unwrap();
                }
                Resource::Contact(_) => {
                    uow.resource_repository()
                        .insert(resource_id.clone(), ResourceType::Contact, 3)
                        .await
                        .unwrap();
                }
                Resource::Article(_) => {
                    uow.resource_repository()
                        .insert(resource_id.clone(), ResourceType::Article, 4)
                        .await
                        .unwrap();
                }
                Resource::Category(_) => {
                    uow.resource_repository()
                        .insert(resource_id.clone(), ResourceType::Category, 5)
                        .await
                        .unwrap();
                }
            };

            uow.content_repository()
                .insert(content_id.clone(), content_data, Language::ZH)
                .await
                .unwrap();

            resources_with_id.push((content_id, resource));
        }

        let _ = uow.avatar_repository();

        (uow, resources_with_id)
    }
}
