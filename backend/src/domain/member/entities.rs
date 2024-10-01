use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AvatarData {
    pub(crate) large_image: String,
    pub(crate) small_image: String,
}

#[derive(Debug, Clone)]
pub struct AvatarJson(serde_json::Value);

impl TryFrom<AvatarData> for AvatarJson {
    type Error = anyhow::Error;

    fn try_from(value: AvatarData) -> anyhow::Result<Self> {
        let json = serde_json::value::to_value(value)?;
        Ok(AvatarJson(json))
    }
}

impl AvatarJson {
    pub fn get(self) -> serde_json::Value {
        self.0
    }
}
//
// #[derive(Debug, Serialize, sqlx::FromRow)]
// pub struct Member {
//     pub member_id: String,
//     pub content: serde_json::Value,
//     pub avatar_data: Option<serde_json::Value>,
// }
//
// #[derive(Debug, Serialize, sqlx::FromRow)]
// pub struct SimpleMember {
//     pub member_id: String,
//     pub name: String,
//     pub avatar: Option<String>,
// }
