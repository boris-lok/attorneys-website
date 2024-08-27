use crate::models::member::Member;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait MemberRepository {
    async fn get_member(&self, id: &str) -> anyhow::Result<Member>;
    async fn get_members(&self) -> anyhow::Result<Vec<Member>>;
    async fn create_member(&mut self, member: Member) -> anyhow::Result<Member>;
}

pub struct InMemoryMemberRepository {
    data: HashMap<String, Member>,
}

impl InMemoryMemberRepository {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Default for InMemoryMemberRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MemberRepository for InMemoryMemberRepository {
    async fn get_member(&self, id: &str) -> anyhow::Result<Member> {
        self.data
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Member not found"))
    }

    async fn get_members(&self) -> anyhow::Result<Vec<Member>> {
        Ok(self.data.values().cloned().collect())
    }

    async fn create_member(&mut self, member: Member) -> anyhow::Result<Member> {
        self.data.insert(member.id.clone(), member.clone());
        Ok(member)
    }
}
