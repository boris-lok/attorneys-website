use crate::domain::role::entity::Role;

#[async_trait::async_trait]
pub trait RoleWriteRepository {}

#[async_trait::async_trait]
pub trait RoleReadRepository {
    async fn list(&mut self) -> Vec<Role>;
}

pub trait RoleRepository: RoleWriteRepository + RoleReadRepository {}
