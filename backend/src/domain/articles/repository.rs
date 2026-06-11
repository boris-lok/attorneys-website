use std::net::IpAddr;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ArticleViewWriteRepository {
    async fn create(&mut self, id: &str, ip: &IpAddr, user_agent: &str) -> anyhow::Result<Uuid>;
}
