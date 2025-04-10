use crate::domain::entities::{Language, Pagination, ResourceType};
use crate::uow::IResourceUnitOfWork;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) struct Request {
    pub(crate) filter_str: Option<String>,
    pub(crate) resource_type: ResourceType,
    pub(crate) language: String,
    pub(crate) default_language: Language,
    pub(crate) pagination: Pagination,
}

pub(crate) enum Error {
    BadRequest,
    Unknown(String),
}

pub async fn execute<IUnitOfWork, T>(
    uow: Mutex<IUnitOfWork>,
    req: Request,
) -> Result<(Vec<T>, usize), Error>
where
    IUnitOfWork: IResourceUnitOfWork,
    T: DeserializeOwned + Serialize,
{
    async fn inner_execute<IUnitOfWork, T>(
        uow: Arc<Mutex<IUnitOfWork>>,
        lang: &Language,
        filter_str: &Option<String>,
        resource_type: &ResourceType,
        pagination: &Pagination,
    ) -> Result<(Vec<T>, usize), Error>
    where
        IUnitOfWork: IResourceUnitOfWork,
        T: DeserializeOwned + Serialize,
    {
        let lock = uow.lock().await;

        let data = lock
            .list_resources::<T>(lang, filter_str, resource_type, pagination)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        let total = match pagination {
            Pagination::All => data.len(),
            Pagination::Single => data.len(),
            Pagination::Page(_) => lock
                .count_resources(lang, resource_type)
                .await
                .map_err(|e| Error::Unknown(e.to_string()))?,
        };

        Ok((data, total))
    }

    let language = Language::try_from(req.language).map_err(|_| Error::BadRequest)?;

    let uow = Arc::new(uow);

    match inner_execute(
        uow.clone(),
        &language,
        &req.filter_str,
        &req.resource_type,
        &req.pagination,
    )
    .await
    {
        Ok((data, total)) => {
            if data.is_empty() {
                inner_execute(
                    uow.clone(),
                    &req.default_language,
                    &req.filter_str,
                    &req.resource_type,
                    &req.pagination,
                )
                .await
            } else {
                Ok((data, total))
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::SimpleMemberEntity;
    use crate::domain::resources::test_helpers::tests::{
        create_resources, create_some_fake_data_and_return_uow,
    };
    use crate::uow::InMemory;

    #[tokio::test]
    async fn it_should_list_resource_otherwise() {
        let (uow, _) = create_some_fake_data_and_return_uow(create_resources()).await;

        let req = Request {
            filter_str: None,
            resource_type: ResourceType::Member,
            language: "zh".to_string(),
            default_language: Language::ZH,
            pagination: Pagination::All,
        };

        let res = execute::<InMemory, SimpleMemberEntity>(Mutex::new(uow), req).await;

        match res {
            Ok((list, total)) => {
                assert_eq!(list.len(), 1);
                assert_eq!(total, 1);
            }
            Err(_) => unreachable!(),
        }
    }
    #[tokio::test]
    async fn it_should_list_default_language_resource_otherwise() {
        let (uow, _) = create_some_fake_data_and_return_uow(create_resources()).await;

        let req = Request {
            filter_str: None,
            resource_type: ResourceType::Member,
            language: "en".to_string(),
            default_language: Language::ZH,
            pagination: Pagination::All,
        };

        let res = execute::<InMemory, SimpleMemberEntity>(Mutex::new(uow), req).await;

        match res {
            Ok((list, total)) => {
                assert_eq!(list.len(), 1);
                assert_eq!(total, 1);
            }
            Err(_) => unreachable!(),
        }
    }
}
