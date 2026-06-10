use crate::domain::users::entity::AvatarData;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct MemberData {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberEntity {
    pub id: String,
    pub language: String,
    pub data: MemberData,
    pub avatar: Option<AvatarData>,
    pub seq: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleMemberEntity {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub seq: i16,
}
