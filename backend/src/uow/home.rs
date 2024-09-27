// use crate::domain::home::entities::{Home, HomeData, HomeID};
// use crate::domain::member::entities::{ContentID, Language};
// use crate::repositories::content_repository::{
//     IContentRepository, InMemoryContentRepository, SqlxContentRepository,
// };
// use crate::repositories::home_repository::{
//     IHomeRepository, InMemoryHomeRepository, SqlxHomeRepository,
// };
// use anyhow::anyhow;
// use sqlx::{PgPool, Postgres, Transaction};
// use std::sync::Arc;
// use tokio::sync::Mutex;
//
// #[async_trait::async_trait]
// pub trait IHomeUnitOfWork {
//     fn home_repository(&mut self) -> &mut impl IHomeRepository;
//     fn content_repository(&mut self) -> &mut impl IContentRepository;
//
//     // Get a service by language with full details
//     async fn get_home<'id, 'lang>(
//         &mut self,
//         home_id: &'id HomeID,
//         language: &'lang Language,
//     ) -> anyhow::Result<Option<Home>>;
//
//     // Get all services for a specific language
//     async fn get_all_home_by_language(&mut self, language: &Language) -> anyhow::Result<Vec<Home>>;
//
//     async fn commit(mut self) -> anyhow::Result<()>;
//     #[allow(dead_code)]
//     async fn rollback(mut self) -> anyhow::Result<()>;
// }
//
// #[derive(Debug)]
// pub struct InMemoryHomeUnitOfWork {
//     error: bool,
//     content_repository: Option<InMemoryContentRepository>,
//     home_repository: Option<InMemoryHomeRepository>,
// }
//
// #[cfg(test)]
// impl InMemoryHomeUnitOfWork {
//     pub fn new() -> Self {
//         Self {
//             error: false,
//             content_repository: None,
//             home_repository: None,
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
// impl IHomeUnitOfWork for InMemoryHomeUnitOfWork {
//     fn home_repository(&mut self) -> &mut impl IHomeRepository {
//         if self.home_repository.is_none() {
//             let service_repo = if self.error {
//                 InMemoryHomeRepository::new().with_error()
//             } else {
//                 InMemoryHomeRepository::new()
//             };
//             self.home_repository = Some(service_repo);
//         }
//         self.home_repository.as_mut().unwrap()
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
//     async fn get_home<'id, 'lang>(
//         &mut self,
//         home_id: &'id HomeID,
//         language: &'lang Language,
//     ) -> anyhow::Result<Option<Home>> {
//         let home = self.home_repository.as_mut().unwrap().get(home_id).await?;
//
//         let content_id = ContentID::try_from(home_id.clone()).unwrap();
//         let content = self
//             .content_repository
//             .as_mut()
//             .unwrap()
//             .get(&content_id, language)
//             .await?;
//
//         match (home, content) {
//             (None, None) => Ok(None),
//             (Some(_), None) => Ok(None),
//             (None, Some(_)) => Ok(None),
//             (Some(h), Some(c)) => {
//                 let data: HomeData = serde_json::value::from_value(c.0)?;
//                 Ok(Some(Home {
//                     home_id: h,
//                     content: data.data,
//                 }))
//             }
//         }
//     }
//
//     async fn get_all_home_by_language(&mut self, language: &Language) -> anyhow::Result<Vec<Home>> {
//         let res = self
//             .content_repository
//             .as_mut()
//             .unwrap()
//             .list::<HomeData>(language)
//             .await?
//             .iter()
//             .map(|(id, content)| Home {
//                 home_id: id.to_string(),
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
// pub struct SqlxHomeUnitOfWork<'tx> {
//     pool: &'tx PgPool,
//     tx: Arc<Mutex<Transaction<'tx, Postgres>>>,
//     content_repository: Option<SqlxContentRepository<'tx>>,
//     home_repository: Option<SqlxHomeRepository<'tx>>,
// }
//
// impl<'tx> SqlxHomeUnitOfWork<'tx> {
//     pub async fn new(pool: &'tx PgPool) -> anyhow::Result<Self> {
//         let tx = pool.begin().await?;
//         let tx = Arc::new(Mutex::new(tx));
//
//         Ok(Self {
//             pool,
//             tx,
//             content_repository: None,
//             home_repository: None,
//         })
//     }
// }
//
// #[async_trait::async_trait]
// impl<'tx> IHomeUnitOfWork for SqlxHomeUnitOfWork<'tx> {
//     fn home_repository(&mut self) -> &mut impl IHomeRepository {
//         if self.home_repository.is_none() {
//             let home_repo = SqlxHomeRepository::new(Arc::downgrade(&self.tx));
//             self.home_repository = Some(home_repo);
//         }
//         self.home_repository.as_mut().unwrap()
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
//     async fn get_home<'id, 'lang>(
//         &mut self,
//         home_id: &'id HomeID,
//         language: &'lang Language,
//     ) -> anyhow::Result<Option<Home>> {
//         let query = r#"
// select home.id as home_id, content.data->>'data' as content
// from home,
//      content
// where home.id = content.id
//   and content.language = $2
//   and home.id = $1;
//         "#;
//
//         let res = sqlx::query_as::<_, Home>(query)
//             .bind(home_id.as_str())
//             .bind(language.as_str())
//             .fetch_optional(self.pool)
//             .await?;
//
//         Ok(res)
//     }
//
//     async fn get_all_home_by_language(&mut self, language: &Language) -> anyhow::Result<Vec<Home>> {
//         let query = r#"select home.id as home_id,
// content.data->>'data' as content
// from home,
//      content
// where home.id = content.id
// and content.language = $1
// order by seq;
//         "#;
//
//         let res = sqlx::query_as::<_, Home>(query)
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
