use crate::domain::session::store::SessionStore;
use crate::domain::uow::common::UnitOfWorkFactory;
use crate::domain::uow::user::UserUoW;
use crate::domain::users::entity::UserID;
use std::sync::Arc;

pub async fn execute<F: UnitOfWorkFactory>(
    uow: &UserUoW<F>,
    session: Arc<dyn SessionStore + Sync + Send>,
    user_id: &UserID,
) -> anyhow::Result<()> {
    uow.delete(user_id).await?;

    session.clear_user_sessions(user_id).await?;

    Ok(())
}
