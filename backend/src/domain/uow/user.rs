use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::domain::users::entity::UserID;
use crate::domain::users::repository::{UserRoleWriteRepository, UserWriteRepository};
use crate::impl_uow;
use secrecy::SecretBox;

impl_uow!(UserUoW);

impl<F: UnitOfWorkFactory> UserUoW<F> {
    pub async fn create(
        &self,
        username: String,
        password_hash: SecretBox<String>,
        nickname: String,
        role_ids: Vec<i16>,
    ) -> anyhow::Result<UserID> {
        let mut uow = self.factory.begin().await?;

        let id = async {
            let user_id = uow
                .user_repo()
                .create(username, password_hash, nickname)
                .await?;

            for role_id in role_ids {
                uow.user_role_repo().create(&user_id, role_id).await?;
            }

            Ok::<_, anyhow::Error>(user_id)
        }
        .await?;

        uow.commit().await?;

        Ok(id)
    }
    pub async fn change_password(
        &self,
        user_id: &UserID,
        password_hash: SecretBox<String>,
    ) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        uow.user_repo()
            .change_password(user_id, password_hash)
            .await?;

        uow.commit().await
    }

    pub async fn delete(&self, user_id: &UserID) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        uow.user_repo().delete(user_id).await?;

        uow.commit().await
    }
}
