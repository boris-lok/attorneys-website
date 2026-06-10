use crate::domain::articles::entity::{ArticleData, CategoryData};
use crate::domain::member::entity::MemberData;
use crate::domain::services::entity::ServiceData;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use validator::Validate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ResourceID(String);

impl TryFrom<String> for ResourceID {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim().is_empty() {
            true => Err(()),
            false => Ok(ResourceID(value)),
        }
    }
}

impl Display for ResourceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResourceID {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct ContentID(String);

impl From<ResourceID> for ContentID {
    fn from(value: ResourceID) -> Self {
        ContentID(value.to_string())
    }
}

impl ContentID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ContentID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ResourceType {
    Member,
    Service,
    Home,
    Contact,
    Article,
    Category,
}

impl ResourceType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Member => "member",
            Self::Service => "services",
            Self::Home => "home",
            Self::Contact => "contact",
            Self::Article => "article",
            Self::Category => "category",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Resource {
    Member(MemberData),
    Service(ServiceData),
    Home(HomeData),
    Contact(ContactData),
    Article(ArticleData),
    Category(CategoryData),
}

#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
    #[error("Failed to validate resource")]
    ValidationError,
    #[error("Failed to serialize resource")]
    SerializationError,
}

impl Resource {
    fn validate(&self) -> Result<(), ResourceError> {
        match self {
            Resource::Member(m) => m.validate(),
            Resource::Service(s) => s.validate(),
            Resource::Home(h) => h.validate(),
            Resource::Contact(c) => c.validate(),
            Resource::Article(a) => a.validate(),
            Resource::Category(c) => c.validate(),
        }
        .map_err(|_| ResourceError::ValidationError)
    }

    /// Converts resource into its type and content data
    pub fn into_typed_content(self) -> Result<(ResourceType, ContentData), ResourceError> {
        let resource_type = ResourceType::from(&self);
        self.validate()?;

        if let Resource::Contact(c) = self {
            Ok((
                resource_type,
                json!({
                    "data": c.data,
                }),
            ))
        } else {
            let content =
                serde_json::value::to_value(self).map_err(|_| ResourceError::SerializationError)?;
            Ok((resource_type, content))
        }
    }
}

impl From<&Resource> for ResourceType {
    fn from(value: &Resource) -> Self {
        match value {
            Resource::Member(_) => ResourceType::Member,
            Resource::Service(_) => ResourceType::Service,
            Resource::Home(_) => ResourceType::Home,
            Resource::Contact(_) => ResourceType::Contact,
            Resource::Article(_) => ResourceType::Article,
            Resource::Category(_) => ResourceType::Category,
        }
    }
}

pub type ContentData = serde_json::Value;

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct HomeData {
    #[validate(length(min = 1))]
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeEntity {
    pub id: String,
    pub language: String,
    pub data: HomeData,
}

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct ContactData {
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactEntity {
    pub id: String,
    pub language: String,
    pub data: serde_json::Value,
}

#[derive(Debug)]
pub struct CreateResourceRequest {
    pub id: ResourceID,
    pub kind: ResourceType,
    pub data: ContentData,
    pub seq: i32,
    pub language: Language,
}

#[derive(Debug)]
pub struct UpdateResourceRequest {
    pub id: ResourceID,
    pub kind: ResourceType,
    pub data: ContentData,
    pub seq: i32,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Language {
    ZH,
    EN,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ZH => "zh",
            Self::EN => "en",
        }
    }
}

impl TryFrom<String> for Language {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "zh" => Ok(Self::ZH),
            "en" => Ok(Self::EN),
            _ => Err(()),
        }
    }
}
