// use crate::domain::home::entities::HomeID;
// use anyhow::anyhow;
// use sqlx::{Acquire, Postgres, Row, Transaction};
// use std::sync::Weak;
// use tokio::sync::Mutex;
//
// #[async_trait::async_trait]
// pub trait IHomeRepository {
//     async fn insert(&self, id: HomeID) -> anyhow::Result<HomeID>;
//     async fn contains(&self, id: &HomeID) -> anyhow::Result<bool>;
// }
//
// #[derive(Debug)]
// pub struct InMemoryHomeRepository {
//     error: bool,
//     home: Mutex<Vec<String>>,
// }
//
// impl InMemoryHomeRepository {
//     pub fn new() -> Self {
//         Self {
//             error: false,
//             home: Mutex::new(Vec::new()),
//         }
//     }
//
//     pub fn with_error(self) -> Self {
//         Self {
//             error: true,
//             ..self
//         }
//     }
//
//     pub async fn get(&self, id: &HomeID) -> anyhow::Result<Option<String>> {
//         if self.error {
//             return Err(anyhow!("Internal Server Error"));
//         }
//
//         let lock = self.home.lock().await;
//
//         Ok(lock
//             .iter()
//             .find(|e| e.as_str() == id.as_str())
//             .map(|e| e.to_owned()))
//     }
// }
//
// #[async_trait::async_trait]
// impl IHomeRepository for InMemoryHomeRepository {
//     async fn insert(&self, id: HomeID) -> anyhow::Result<HomeID> {
//         if self.error {
//             return Err(anyhow!("Internal Server Error"));
//         }
//
//         let mut lock = self.home.lock().await;
//
//         if lock.iter().any(|id| id == id.as_str()) {
//             return Err(anyhow!("{} already exists", id));
//         }
//
//         lock.push(id.to_string());
//         Ok(id)
//     }
//
//     async fn contains(&self, id: &HomeID) -> anyhow::Result<bool> {
//         if self.error {
//             return Err(anyhow!("Internal Server Error"));
//         }
//
//         let lock = self.home.lock().await;
//         Ok(lock.iter().any(|hid| hid.as_str() == id.as_str()))
//     }
// }
//
// #[derive(Debug)]
// pub struct SqlxHomeRepository<'tx> {
//     tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
// }
//
// impl<'tx> SqlxHomeRepository<'tx> {
//     pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
//         Self { tx }
//     }
// }
//
// #[async_trait::async_trait]
// impl<'tx> IHomeRepository for SqlxHomeRepository<'tx> {
//     async fn insert(&self, id: HomeID) -> anyhow::Result<HomeID> {
//         let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
//         let mut lock = conn_ptr.lock().await;
//         let conn = lock.acquire().await?;
//
//         sqlx::query("INSERT INTO \"home\" (id, created_at, seq) VALUES ($1, now(), 32767 );")
//             .bind(id.as_str())
//             .execute(conn)
//             .await?;
//
//         Ok(id)
//     }
//
//     async fn contains(&self, id: &HomeID) -> anyhow::Result<bool> {
//         let query = "SELECT id FROM \"home\" WHERE id = $1 limit 1";
//
//         let conn_ptr = self.tx.upgrade().ok_or(anyhow!("Internal Server Error"))?;
//         let mut lock = conn_ptr.lock().await;
//         let conn = lock.acquire().await?;
//
//         let res = sqlx::query(query)
//             .bind(id.as_str())
//             .fetch_optional(conn)
//             .await
//             .map(|row| match row {
//                 None => false,
//                 Some(row) => row.len() > 0,
//             })?;
//
//         Ok(res)
//     }
// }
