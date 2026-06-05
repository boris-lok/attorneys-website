use crate::domain::users::entity::{User, UserID};
use secrecy::SecretBox;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create(
        &mut self,
        username: String,
        password_hash: SecretBox<String>,
        nickname: String,
    ) -> anyhow::Result<UserID>;

    async fn list(&mut self) -> anyhow::Result<Vec<User>>;

    async fn delete(&mut self, id: &UserID) -> anyhow::Result<()>;

    async fn get_user_nickname(&mut self, id: &UserID) -> anyhow::Result<String>;

    async fn get_credentials(
        &mut self,
        username: &str,
    ) -> anyhow::Result<Option<(UserID, SecretBox<String>)>>;

    async fn change_password(
        &mut self,
        id: &UserID,
        password_hash: SecretBox<String>,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait UserRoleRepository {
    async fn get_user_roles(&mut self, id: &UserID) -> anyhow::Result<Vec<String>>;

    async fn create(&mut self, user_id: &UserID, role: i16) -> anyhow::Result<()>;
}
