use crate::domain::resources::entity::{ContentData, ContentID, Language};
use crate::domain::resources::repository::ContentWriteRepository;
use crate::infrastructure::db::connection::{ContentRepo, PostgresRepo};

const CREATE_CONTENT_QUERY: &str = r"
  INSERT INTO 'content' (id, language, data, created_at, updated_at) VALUES ($1, $2, $3, now(), now());\
";

const UPDATE_CONTENT_QUERY: &str = r"
  UPDATE 'content' SET data = $1, updated_at = now() WHERE id = $2 AND language = $3;
";

type PostgresContentRepo<'tx> = PostgresRepo<'tx, ContentRepo>;

#[async_trait::async_trait]
impl<'tx> ContentWriteRepository for PostgresContentRepo<'tx> {
    async fn create(
        &mut self,
        id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()> {
        sqlx::query(CREATE_CONTENT_QUERY)
            .bind(id.as_str())
            .bind(language.as_str())
            .bind(data)
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn update(
        &mut self,
        id: &ContentID,
        data: ContentData,
        language: Language,
    ) -> anyhow::Result<()> {
        sqlx::query(UPDATE_CONTENT_QUERY)
            .bind(data)
            .bind(id.as_str())
            .bind(language.as_str())
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }
}
