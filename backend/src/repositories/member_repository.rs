use crate::domain::entities::{Language, Member, MemberID, MemberName};

pub enum Insert {
    Ok(MemberID),
    Conflict,
    Error,
}

pub trait MemberRepository {
    fn insert(
        &mut self,
        member_id: MemberID,
        member_name: MemberName,
        default_language: Language,
    ) -> Insert;
}

pub struct InMemoryMemberRepository {
    error: bool,
    members: Vec<Member>,
}

impl InMemoryMemberRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            members: Vec::new(),
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

impl MemberRepository for InMemoryMemberRepository {
    fn insert(
        &mut self,
        member_id: MemberID,
        member_name: MemberName,
        default_language: Language,
    ) -> Insert {
        if self.error {
            return Insert::Error;
        }

        if self.members.iter().any(|m| m.member_id == member_id) {
            return Insert::Conflict;
        }

        let member_id_cloned = member_id.clone();
        self.members
            .push(Member::new(member_id_cloned, member_name, default_language));
        Insert::Ok(member_id)
    }
}
