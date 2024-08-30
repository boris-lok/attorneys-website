use crate::domain::entities::{Language, Member, MemberID, MemberName};
use std::sync::Mutex;

pub enum Insert {
    Ok(MemberID),
    Conflict,
    Error,
}

pub trait MemberRepository {
    fn insert(&self, member_id: MemberID, member_name: MemberName, language: Language) -> Insert;
}

pub struct InMemoryMemberRepository {
    error: bool,
    members: Mutex<Vec<Member>>,
}

impl InMemoryMemberRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            members: Mutex::new(Vec::new()),
        }
    }

    #[cfg(test)]
    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

impl MemberRepository for InMemoryMemberRepository {
    fn insert(&self, member_id: MemberID, member_name: MemberName, _: Language) -> Insert {
        if self.error {
            return Insert::Error;
        }

        let mut lock = match self.members.lock() {
            Ok(lock) => lock,
            _ => return Insert::Error,
        };

        if lock.iter().any(|m| m.member_id == member_id) {
            return Insert::Conflict;
        }

        let member_id_cloned = member_id.clone();
        lock.push(Member::new(member_id_cloned, member_name));
        Insert::Ok(member_id)
    }
}
