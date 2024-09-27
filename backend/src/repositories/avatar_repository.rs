use crate::domain::entities::ResourceID;
use crate::domain::member::entities::AvatarJson;

#[derive(Debug)]
pub enum InsertError {
    Conflict,
    Unknown,
}
#[async_trait::async_trait]
pub trait IAvatarRepository {
    async fn insert(
        &self,
        avatar_id: ResourceID,
        avatar_json: AvatarJson,
    ) -> Result<ResourceID, InsertError>;
}

// #[derive(Debug)]
// pub struct InMemoryAvatarRepository {
//     error: bool,
//     content: Mutex<HashMap<String, AvatarJson>>,
// }
//
// impl InMemoryAvatarRepository {
//     pub fn new() -> Self {
//         Self {
//             error: false,
//             content: Mutex::new(HashMap::new()),
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
//     pub async fn get(&self, id: &MemberID) -> anyhow::Result<Option<AvatarJson>> {
//         if self.error {
//             return Err(anyhow!("Internal Server Error"));
//         }
//
//         let lock = self.content.lock().await;
//
//         Ok(lock.get(id.as_str()).cloned())
//     }
// }
//
// #[async_trait::async_trait]
// impl IAvatarRepository for InMemoryAvatarRepository {
//     async fn insert(
//         &self,
//         avatar_id: MemberID,
//         avatar_json: AvatarJson,
//     ) -> Result<MemberID, InsertError> {
//         if self.error {
//             return Err(InsertError::Unknown);
//         }
//
//         let mut lock = self.content.lock().await;
//
//         let key = avatar_id.to_string();
//         if lock.contains_key(&key) {
//             return Err(InsertError::Conflict);
//         }
//
//         lock.insert(key, avatar_json);
//
//         Ok(avatar_id)
//     }
// }
//
// #[derive(Debug)]
// pub struct SqlxAvatarRepository<'tx> {
//     tx: Weak<Mutex<Transaction<'tx, Postgres>>>,
// }
//
// impl<'tx> SqlxAvatarRepository<'tx> {
//     pub fn new(tx: Weak<Mutex<Transaction<'tx, Postgres>>>) -> Self {
//         Self { tx }
//     }
// }
//
// #[async_trait::async_trait]
// impl<'tx> IAvatarRepository for SqlxAvatarRepository<'tx> {
//     async fn insert(
//         &self,
//         member_id: MemberID,
//         avatar_json: AvatarJson,
//     ) -> Result<MemberID, InsertError> {
//         let conn_ptr = self.tx.upgrade().ok_or(InsertError::Unknown)?;
//         let mut lock = conn_ptr.lock().await;
//         let conn = lock.acquire().await.unwrap();
//
//         sqlx::query("INSERT INTO \"avatar\" (id, data) VALUES ($1, $2); ")
//             .bind(member_id.as_str())
//             .bind(avatar_json.get())
//             .execute(conn)
//             .await
//             .map_err(|e| {
//                 println!("Failed to insert avatar: {:?}", e);
//
//                 InsertError::Unknown
//             })?;
//
//         Ok(member_id)
//     }
// }
