use crate::domain::role::entity::Role;

#[async_trait::async_trait]
pub trait RoleRepository {
    async fn list(&mut self) -> Vec<Role>;
}
