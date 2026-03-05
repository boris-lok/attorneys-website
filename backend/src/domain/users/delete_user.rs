use crate::domain::entities::UserID;
use crate::repositories::IUserRepository;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Request {
    pub id: UserID,
}

pub async fn execute(
    req: Request,
    user_repo: Mutex<impl IUserRepository + Sync + Send>,
) -> anyhow::Result<()> {
    user_repo.lock().await.delete_user(&req.id).await
}
