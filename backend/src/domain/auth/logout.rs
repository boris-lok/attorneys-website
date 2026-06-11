use crate::domain::session::store::SessionStore;
use crate::domain::users::entity::UserID;
use std::sync::Arc;

pub async fn execute(
    session: Arc<dyn SessionStore + Send + Sync>,
    user_id: &UserID,
) -> anyhow::Result<()> {
    session.clear_user_sessions(user_id).await
}
