// use crate::domain::member::entities::{ContentID, Language};
// use crate::domain::service::entities::{Service, ServiceData, ServiceID};
// use crate::repositories::content_repository::{
//     IContentRepository, InMemoryContentRepository, SqlxContentRepository,
// };
// use crate::repositories::service_repository::{
//     IServiceRepository, InMemoryServiceRepository, SqlxServiceRepository,
// };
// use anyhow::anyhow;
// use sqlx::{PgPool, Postgres, Transaction};
// use std::sync::Arc;
// use tokio::sync::Mutex;
//
// #[async_trait::async_trait]
// pub trait IServiceUnitOfWork {
//     fn service_repository(&mut self) -> &mut impl IServiceRepository;
//     fn content_repository(&mut self) -> &mut impl IContentRepository;
//
//     // Get a service by language with full details
//     async fn get_service<'id, 'lang>(
//         &mut self,
//         service_id: &'id ServiceID,
//         language: &'lang Language,
//     ) -> anyhow::Result<Option<Service>>;
//
//     // Get all services for a specific language
//     async fn get_all_services_by_language(
//         &mut self,
//         language: &Language,
//     ) -> anyhow::Result<Vec<Service>>;
//
//     async fn commit(mut self) -> anyhow::Result<()>;
//     #[allow(dead_code)]
//     async fn rollback(mut self) -> anyhow::Result<()>;
// }
//
// #[derive(Debug)]
// pub struct InMemoryServiceUnitOfWork {
//     error: bool,
//     content_repository: Option<InMemoryContentRepository>,
//     service_repository: Option<InMemoryServiceRepository>,
// }
//
// #[cfg(test)]
// impl InMemoryServiceUnitOfWork {
//     pub fn new() -> Self {
//         Self {
//             error: false,
//             content_repository: None,
//             service_repository: None,
//         }
//     }
//
//     pub fn with_error(mut self) -> Self {
//         self.content_repository = self.content_repository.map(|r| r.with_error());
//         Self {
//             error: true,
//             ..self
//         }
//     }
// }
//
// #[async_trait::async_trait]
// impl IServiceUnitOfWork for InMemoryServiceUnitOfWork {
//     fn service_repository(&mut self) -> &mut impl IServiceRepository {
//         if self.service_repository.is_none() {
//             let service_repo = if self.error {
//                 InMemoryServiceRepository::new().with_error()
//             } else {
//                 InMemoryServiceRepository::new()
//             };
//             self.service_repository = Some(service_repo);
//         }
//         self.service_repository.as_mut().unwrap()
//     }
//
//     fn content_repository(&mut self) -> &mut impl IContentRepository {
//         if self.content_repository.is_none() {
//             let content_repo = if self.error {
//                 InMemoryContentRepository::new().with_error()
//             } else {
//                 InMemoryContentRepository::new()
//             };
//             self.content_repository = Some(content_repo);
//         }
//         self.content_repository.as_mut().unwrap()
//     }
//
//     async fn get_service<'id, 'lang>(
//         &mut self,
//         service_id: &'id ServiceID,
//         language: &'lang Language,
//     ) -> anyhow::Result<Option<Service>> {
//         let service = self
//             .service_repository
//             .as_mut()
//             .unwrap()
//             .get(service_id)
//             .await?;
//
//         let content_id = ContentID::try_from(service_id.clone()).unwrap();
//         let content = self
//             .content_repository
//             .as_mut()
//             .unwrap()
//             .get(&content_id, language)
//             .await?;
//
//         match (service, content) {
//             (None, None) => Ok(None),
//             (Some(_), None) => Ok(None),
//             (None, Some(_)) => Ok(None),
//             (Some(s), Some(c)) => {
//                 let data: ServiceData = serde_json::value::from_value(c.0)?;
//                 Ok(Some(Service {
//                     service_id: s,
//                     content: data.data,
//                 }))
//             }
//         }
//     }
//
//     async fn get_all_services_by_language(
//         &mut self,
//         language: &Language,
//     ) -> anyhow::Result<Vec<Service>> {
//         let res = self
//             .content_repository
//             .as_mut()
//             .unwrap()
//             .list::<ServiceData>(language)
//             .await?
//             .iter()
//             .map(|(id, content)| Service {
//                 service_id: id.to_string(),
//                 content: content.data.to_string(),
//             })
//             .collect::<Vec<_>>();
//
//         Ok(res)
//     }
//
//     async fn commit(self) -> anyhow::Result<()> {
//         Ok(())
//     }
//
//     async fn rollback(self) -> anyhow::Result<()> {
//         Ok(())
//     }
// }
//
// #[derive(Debug)]
// pub struct SqlxServiceUnitOfWork<'tx> {
//     pool: &'tx PgPool,
//     tx: Arc<Mutex<Transaction<'tx, Postgres>>>,
//     content_repository: Option<SqlxContentRepository<'tx>>,
//     service_repository: Option<SqlxServiceRepository<'tx>>,
// }
//
// impl<'tx> SqlxServiceUnitOfWork<'tx> {
//     pub async fn new(pool: &'tx PgPool) -> anyhow::Result<Self> {
//         let tx = pool.begin().await?;
//         let tx = Arc::new(Mutex::new(tx));
//
//         Ok(Self {
//             pool,
//             tx,
//             content_repository: None,
//             service_repository: None,
//         })
//     }
// }
//
// #[async_trait::async_trait]
// impl<'tx> IServiceUnitOfWork for SqlxServiceUnitOfWork<'tx> {
//     fn service_repository(&mut self) -> &mut impl IServiceRepository {
//         if self.service_repository.is_none() {
//             let service_repo = SqlxServiceRepository::new(Arc::downgrade(&self.tx));
//             self.service_repository = Some(service_repo);
//         }
//         self.service_repository.as_mut().unwrap()
//     }
//
//     fn content_repository(&mut self) -> &mut impl IContentRepository {
//         if self.content_repository.is_none() {
//             // Use Arc::downgrade to obtain a weak reference to the transaction
//             // if we don't do this, when we call the commit/rollback method will fail.
//             // It can't `try_unwrap` because there are at least two strong references, preventing
//             // the use of `try_unwrap`.
//             //
//             // If we want to use strong references, then we need to drop the repository
//             // when we try to call commit/rollback methods.
//             let content_repo = SqlxContentRepository::new(Arc::downgrade(&self.tx));
//             self.content_repository = Some(content_repo);
//         }
//         self.content_repository.as_mut().unwrap()
//     }
//
//     async fn get_service<'id, 'lang>(
//         &mut self,
//         service_id: &'id ServiceID,
//         language: &'lang Language,
//     ) -> anyhow::Result<Option<Service>> {
//         let query = r#"
// select service.id as service_id, content.data->>'data' as content
// from service,
//      content
// where service.id = content.id
//   and content.language = $2
//   and service.id = $1;
//         "#;
//
//         let res = sqlx::query_as::<_, Service>(query)
//             .bind(service_id.as_str())
//             .bind(language.as_str())
//             .fetch_optional(self.pool)
//             .await?;
//
//         Ok(res)
//     }
//
//     async fn get_all_services_by_language(
//         &mut self,
//         language: &Language,
//     ) -> anyhow::Result<Vec<Service>> {
//         let query = r#"select service.id as service_id,
// content.data->>'data' as content
// from service,
//      content
// where service.id = content.id
// and content.language = $1
// order by seq;
//         "#;
//
//         let res = sqlx::query_as::<_, Service>(query)
//             .bind(language.as_str())
//             .fetch_all(self.pool)
//             .await?;
//
//         Ok(res)
//     }
//
//     async fn commit(mut self) -> anyhow::Result<()> {
//         match Arc::try_unwrap(self.tx) {
//             Ok(lock) => {
//                 lock.into_inner().commit().await?;
//                 Ok(())
//             }
//             Err(_) => Err(anyhow!("can't commit transaction")),
//         }
//     }
//
//     async fn rollback(mut self) -> anyhow::Result<()> {
//         match Arc::try_unwrap(self.tx) {
//             Ok(lock) => {
//                 lock.into_inner().rollback().await?;
//                 Ok(())
//             }
//             Err(_) => Err(anyhow!("can't rollback transaction")),
//         }
//     }
// }
