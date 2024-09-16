use crate::domain::home::entities::{HomeData, HomeID};
use crate::domain::service::entities::{ServiceData, ServiceID};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub enum Language {
    ZH,
    EN,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct MemberID(String);

impl TryFrom<String> for MemberID {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.is_empty() {
            true => Err(()),
            false => Ok(MemberID(value)),
        }
    }
}

impl Display for MemberID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl MemberID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Language {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "en" => Ok(Language::EN),
            "zh" => Ok(Language::ZH),
            _ => Err(()),
        }
    }
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::ZH => "zh",
            Language::EN => "en",
        }
    }
}
#[derive(Debug, Serialize, Clone, Deserialize)]
pub(crate) struct MemberData {
    pub(crate) name: String,
    pub(crate) description: String,
}

impl TryFrom<Data> for MemberData {
    type Error = ();

    fn try_from(value: Data) -> Result<Self, Self::Error> {
        let name = value.name.trim().to_string();
        let description = value.description.trim().to_string();
        if name.is_empty() || description.is_empty() {
            Err(())
        } else {
            Ok(MemberData { name, description })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContentID(String);

impl TryFrom<MemberID> for ContentID {
    type Error = ();

    fn try_from(value: MemberID) -> Result<Self, Self::Error> {
        Ok(ContentID(value.0))
    }
}

impl TryFrom<ServiceID> for ContentID {
    type Error = ();

    fn try_from(value: ServiceID) -> Result<Self, Self::Error> {
        Ok(ContentID(value.as_str().to_string()))
    }
}
impl TryFrom<HomeID> for ContentID {
    type Error = ();

    fn try_from(value: HomeID) -> Result<Self, Self::Error> {
        Ok(ContentID(value.as_str().to_string()))
    }
}

impl Display for ContentID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl ContentID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Data {
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Debug, Clone)]
pub struct ContentData(pub(crate) serde_json::Value);

impl TryFrom<MemberData> for ContentData {
    type Error = anyhow::Error;

    fn try_from(value: MemberData) -> anyhow::Result<Self> {
        let data = serde_json::value::to_value(&value)?;
        Ok(ContentData(data))
    }
}

impl TryFrom<ServiceData> for ContentData {
    type Error = anyhow::Error;

    fn try_from(value: ServiceData) -> anyhow::Result<Self> {
        let data = serde_json::value::to_value(&value)?;
        Ok(ContentData(data))
    }
}

impl TryFrom<HomeData> for ContentData {
    type Error = anyhow::Error;

    fn try_from(value: HomeData) -> anyhow::Result<Self> {
        let data = serde_json::value::to_value(&value)?;
        Ok(ContentData(data))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Member {
    pub member_id: String,
    pub content: serde_json::Value,
    pub avatar_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SimpleMember {
    pub member_id: String,
    pub name: String,
    pub avatar: Option<String>,
}
