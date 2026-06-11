use crate::domain::uow::common::{UnitOfWork, UnitOfWorkFactory};
use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::repository::WorkLogMappingRepository;
use crate::domain::work_logs::entity::CreateWorkLogRequest;
use crate::domain::work_logs::repository::WorkLogsRepository;
use crate::impl_uow;

impl_uow!(WorkLogUoW);

impl<F: UnitOfWorkFactory> WorkLogUoW<F> {
    pub async fn create(
        &self,
        log: CreateWorkLogRequest,
        collaborator_ids: Vec<UserID>,
    ) -> anyhow::Result<()> {
        let mut uow = self.factory.begin().await?;

        let res = async {
            let id = log.id;
            uow.work_log_repo().create(log).await?;
            uow.work_log_mapping_repo()
                .create(&id, collaborator_ids)
                .await
        }
        .await;

        match res {
            Ok(_) => uow.commit().await?,
            Err(e) => {
                uow.rollback().await?;
                return Err(e);
            }
        }

        Ok(())
    }
}
