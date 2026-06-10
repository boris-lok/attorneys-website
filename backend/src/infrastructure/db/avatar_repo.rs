use crate::domain::resources::entity::ResourceID;
use crate::domain::users::entity::AvatarJson;
use crate::domain::users::repository::AvatarRepository;
use crate::infrastructure::db::connection::{AvatarRepo, PostgresRepo};

const CREATE_AVATAR_QUERY: &str = r"
  INSERT INTO avatars (id, json) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = $3;
";

type PostgresAvatarRepo<'tx> = PostgresRepo<'tx, AvatarRepo>;

#[async_trait::async_trait]
impl<'tx> AvatarRepository for PostgresAvatarRepo<'tx> {
    async fn create(&mut self, id: &ResourceID, json: AvatarJson) -> anyhow::Result<()> {
        sqlx::query(CREATE_AVATAR_QUERY)
            .bind(id.as_str())
            .bind(json.clone().get())
            .bind(json.get())
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }
}
