use crate::domain::articles::repository::ArticleViewRepository;
use crate::infrastructure::db::connection::{ArticleViewRepo, PostgresRepo};
use std::net::IpAddr;
use uuid::Uuid;

const CREATE_ARTICLES_QUERY: &str = r"
  INSERT INTO articles_views (article_id, ip, user_agent) VALUES ($1, $2, $3) RETURNING id;
";

type PostgresArticleViewRepo<'tx> = PostgresRepo<'tx, ArticleViewRepo>;

#[async_trait::async_trait]
impl<'tx> ArticleViewRepository for PostgresArticleViewRepo<'tx> {
    async fn create(&mut self, id: &str, ip: &IpAddr, user_agent: &str) -> anyhow::Result<Uuid> {
        let id = sqlx::query_scalar::<_, Uuid>(CREATE_ARTICLES_QUERY)
            .bind(id)
            .bind(ip)
            .bind(user_agent)
            .fetch_one(self.get_conn())
            .await?;

        Ok(id)
    }
}
