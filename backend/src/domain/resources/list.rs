use crate::domain::entity::Pagination;
use crate::domain::resources::entity::{Language, ResourceType};
use crate::domain::resources::repository::ResourceReadRepository;
use crate::domain::uow::common::Query;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub struct Request {
    pub filter_str: Option<String>,
    pub kind: ResourceType,
    pub language: Language,
    pub default_language: Language,
    pub pagination: Pagination,
}

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub async fn execute<T>(query: &impl Query, req: Request) -> Result<(Vec<T>, usize), Error>
where
    T: DeserializeOwned + Serialize,
{
    let filter_str = req.filter_str.unwrap_or_default();

    // Try preferred language first, fall back to default if empty
    let (data, total) = fetch(
        query,
        &req.language,
        &filter_str,
        &req.kind,
        &req.pagination,
    )
    .await?;

    if data.is_empty() {
        fetch(
            query,
            &req.default_language,
            &filter_str,
            &req.kind,
            &req.pagination,
        )
        .await
    } else {
        Ok((data, total))
    }
}

async fn fetch<T>(
    query: &impl Query,
    lang: &Language,
    filter_str: &str,
    kind: &ResourceType,
    pagination: &Pagination,
) -> Result<(Vec<T>, usize), Error>
where
    T: DeserializeOwned + Serialize,
{
    let mut repo = query.resource_repo();

    let data: Vec<T> = repo
        .list(lang, kind, filter_str, pagination)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    let total = if let Pagination::Page(_) = pagination {
        repo.count(lang, kind, filter_str)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))? as usize
    } else {
        data.len()
    };

    Ok((data, total))
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::entity::SimpleMemberEntity;
//     use crate::domain::resources::test_helpers::tests::{
//         create_resources, create_some_fake_data_and_return_uow,
//     };
//     use crate::uow::InMemory;
//
//     #[tokio::test]
//     async fn it_should_list_resource_otherwise() {
//         let (uow, _) = create_some_fake_data_and_return_uow(create_resources()).await;
//
//         let req = Request {
//             filter_str: None,
//             resource_type: ResourceType::Member,
//             language: "zh".to_string(),
//             default_language: Language::ZH,
//             pagination: Pagination::All,
//         };
//
//         let res = execute::<InMemory, SimpleMemberEntity>(Mutex::new(uow), req).await;
//
//         match res {
//             Ok((list, total)) => {
//                 assert_eq!(list.len(), 1);
//                 assert_eq!(total, 1);
//             }
//             Err(_) => unreachable!(),
//         }
//     }
//     #[tokio::test]
//     async fn it_should_list_default_language_resource_otherwise() {
//         let (uow, _) = create_some_fake_data_and_return_uow(create_resources()).await;
//
//         let req = Request {
//             filter_str: None,
//             resource_type: ResourceType::Member,
//             language: "en".to_string(),
//             default_language: Language::ZH,
//             pagination: Pagination::All,
//         };
//
//         let res = execute::<InMemory, SimpleMemberEntity>(Mutex::new(uow), req).await;
//
//         match res {
//             Ok((list, total)) => {
//                 assert_eq!(list.len(), 1);
//                 assert_eq!(total, 1);
//             }
//             Err(_) => unreachable!(),
//         }
//     }
// }
