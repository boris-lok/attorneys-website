use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
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

impl std::fmt::Display for ResourceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResourceID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
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

#[derive(Debug, Clone)]
pub struct ContentID(String);

impl From<ResourceID> for ContentID {
    fn from(value: ResourceID) -> Self {
        ContentID(value.0)
    }
}

impl ContentID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ContentID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ContentData(serde_json::Value);

impl ContentData {
    pub fn to_json(self) -> serde_json::Value {
        self.0
    }

    pub fn as_json(&self) -> &serde_json::Value {
        &self.0
    }
}

impl TryFrom<Resource> for ContentData {
    type Error = ();

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        match resource {
            Resource::Member(m) => try_parse_to_value(m),
            Resource::Service(s) => try_parse_to_value(s),
            Resource::Home(h) => try_parse_to_value(h),
            Resource::Contact(c) => try_parse_to_value(c),
        }
    }
}

fn try_parse_to_value<T>(value: T) -> Result<ContentData, ()>
where
    T: Validate + Serialize,
{
    match value.validate() {
        Ok(_) => match serde_json::value::to_value(value) {
            Ok(v) => Ok(ContentData(v)),
            Err(_) => Err(()),
        },
        Err(_) => Err(()),
    }
}

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct MemberData {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
}

impl MemberData {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name: name.trim().to_string(),
            description: description.trim().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct ServiceData {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub data: String,
}

impl ServiceData {
    pub fn new(title: String, data: String) -> Self {
        Self {
            title: title.trim().to_string(),
            data: data.trim().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct HomeData {
    #[validate(length(min = 1))]
    pub data: String,
}

impl HomeData {
    pub fn new(data: String) -> Self {
        Self {
            data: data.trim().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct ContactData {
    #[validate(length(min = 1))]
    pub address: String,
    #[validate(length(min = 1))]
    pub phone: String,
    #[validate(email)]
    pub email: String,
}

impl ContactData {
    pub fn new(address: String, phone: String, email: String) -> Self {
        Self {
            address: address.trim().to_string(),
            phone: phone.trim().to_string(),
            email: email.trim().to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ResourceType {
    Member,
    Service,
    Home,
    Contact,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Resource {
    Member(MemberData),
    Service(ServiceData),
    Home(HomeData),
    Contact(ContactData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberEntity {
    pub id: String,
    pub language: String,
    pub data: MemberData,
    pub avatar: Option<String>,
}

impl MemberEntity {
    pub fn new(id: String, language: String, data: MemberData, avatar: Option<String>) -> Self {
        Self {
            id,
            language,
            data,
            avatar,
        }
    }
}
