use crate::domain::users::entity::UserID;
use crate::domain::users::repository::UserRepository;

pub async fn execute(repo: &mut impl UserRepository, user_id: &UserID) -> anyhow::Result<()> {
    repo.delete(&user_id).await?;
    // TODO: clear the session.

    Ok(())
}
