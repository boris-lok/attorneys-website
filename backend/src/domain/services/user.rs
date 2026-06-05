use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::domain::users::entity::UserID;
use crate::domain::users::repository::{UserRepository, UserRoleRepository};
use crate::impl_service;
use secrecy::SecretBox;

impl_service!(UserService);

impl<F: UnitOfWorkFactory> UserService<F> {
    pub async fn create(
        &self,
        username: String,
        password_hash: SecretBox<String>,
        nickname: String,
        role_ids: Vec<i16>,
    ) -> anyhow::Result<UserID> {
        let mut uow = self.factory.begin().await?;

        let res = async {
            let user_id = uow
                .user_repo()
                .create(username, password_hash, nickname)
                .await?;

            for role_id in role_ids {
                uow.user_role_repo().create(&user_id, role_id).await?;
            }

            Ok(user_id)
        }
        .await;

        match res {
            Ok(id) => {
                uow.commit().await?;
                Ok(id)
            }
            Err(e) => {
                uow.rollback().await?;
                Err(e)
            }
        }
    }
}
