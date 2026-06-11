use crate::domain::resources::entity::ResourceID;
use crate::domain::users::entity::{AvatarJson, User, UserID};
use secrecy::SecretBox;

#[async_trait::async_trait]
pub trait UserWriteRepository {
    async fn create(
        &mut self,
        username: String,
        password_hash: SecretBox<String>,
        nickname: String,
    ) -> anyhow::Result<UserID>;
    async fn delete(&mut self, id: &UserID) -> anyhow::Result<()>;
    async fn change_password(
        &mut self,
        id: &UserID,
        password_hash: SecretBox<String>,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait UserReadRepository {
    async fn list(&mut self) -> anyhow::Result<Vec<User>>;
    async fn get_user_nickname(&mut self, id: &UserID) -> anyhow::Result<String>;
    async fn get_credentials(
        &mut self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>>;
}

pub trait UserRepository: UserWriteRepository + UserReadRepository {}

#[async_trait::async_trait]
pub trait UserRoleWriteRepository {
    async fn create(&mut self, user_id: &UserID, role: i16) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait UserRoleReadRepository {
    async fn get_user_roles(&mut self, id: &UserID) -> anyhow::Result<Vec<String>>;
}

pub trait UserRoleRepository: UserRoleWriteRepository + UserRoleReadRepository {}

#[async_trait::async_trait]
pub trait AvatarWriteRepository {
    async fn create(&mut self, id: &ResourceID, json: AvatarJson) -> anyhow::Result<()>;
}
