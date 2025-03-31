use crate::domain::member::entities::AvatarData;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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
    pub fn to_json(&self) -> serde_json::Value {
        self.0.clone()
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
            Resource::Article(a) => try_parse_to_value(a),
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

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct ArticleData {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

impl ArticleData {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title: title.trim().to_string(),
            content: content.trim().to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ResourceType {
    Member,
    Service,
    Home,
    Contact,
    Article,
}

impl ResourceType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Member => "member",
            Self::Service => "service",
            Self::Home => "home",
            Self::Contact => "contact",
            Self::Article => "article",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Resource {
    Member(MemberData),
    Service(ServiceData),
    Home(HomeData),
    Contact(ContactData),
    Article(ArticleData),
}

impl Resource {
    /// Convert a Resource to resource type and content data
    pub fn to_resource_type_and_content_data(&self) -> Result<(ResourceType, ContentData), ()> {
        match self {
            Resource::Member(_) => Ok((ResourceType::Member, ContentData::try_from(self.clone())?)),
            Resource::Service(_) => {
                Ok((ResourceType::Service, ContentData::try_from(self.clone())?))
            }
            Resource::Home(_) => Ok((ResourceType::Home, ContentData::try_from(self.clone())?)),
            Resource::Contact(_) => {
                Ok((ResourceType::Contact, ContentData::try_from(self.clone())?))
            }
            Resource::Article(_) => {
                Ok((ResourceType::Article, ContentData::try_from(self.clone())?))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberEntity {
    pub id: String,
    pub language: String,
    pub data: MemberData,
    pub avatar: Option<AvatarData>,
    pub seq: i16,
}

impl MemberEntity {
    pub fn new(
        id: String,
        language: String,
        data: MemberData,
        avatar: Option<AvatarData>,
        seq: i16,
    ) -> Self {
        Self {
            id,
            language,
            data,
            avatar,
            seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct MemberEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<MemberData>,
    pub avatar: Option<sqlx::types::Json<AvatarData>>,
    pub seq: i16,
}

impl From<MemberEntityFromSQLx> for MemberEntity {
    fn from(value: MemberEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
            avatar: value.avatar.map(|a| a.0),
            seq: value.seq,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceEntity {
    pub id: String,
    pub language: String,
    pub data: ServiceData,
    pub seq: i16,
}

impl ServiceEntity {
    pub fn new(id: String, language: String, data: ServiceData, seq: i16) -> Self {
        Self {
            id,
            language,
            data,
            seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ServiceEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<ServiceData>,
    pub seq: i16,
}

impl From<ServiceEntityFromSQLx> for ServiceEntity {
    fn from(value: ServiceEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
            seq: value.seq,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeEntity {
    pub id: String,
    pub language: String,
    pub data: HomeData,
}

impl HomeEntity {
    pub fn new(id: String, language: String, data: HomeData) -> Self {
        Self { id, language, data }
    }
}

#[derive(Debug, FromRow)]
pub struct HomeEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<HomeData>,
}

impl From<HomeEntityFromSQLx> for HomeEntity {
    fn from(value: HomeEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactEntity {
    pub id: String,
    pub language: String,
    pub data: ContactData,
}

impl ContactEntity {
    pub fn new(id: String, language: String, data: ContactData) -> Self {
        Self { id, language, data }
    }
}

#[derive(Debug, FromRow)]
pub struct ContactEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<ContactData>,
}

impl From<ContactEntityFromSQLx> for ContactEntity {
    fn from(value: ContactEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleEntity {
    pub id: String,
    pub language: String,
    pub data: ArticleData,
    pub seq: i16,
}

impl ArticleEntity {
    pub fn new(id: String, language: String, data: ArticleData, seq: i16) -> Self {
        Self {
            id,
            language,
            data,
            seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ArticleEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<ArticleData>,
    pub seq: i16,
}

impl From<ArticleEntityFromSQLx> for ArticleEntity {
    fn from(value: ArticleEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
            seq: value.seq,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleArticleEntity {
    pub id: String,
    pub title: String,
    pub language: String,
    pub created_at: i64,
    pub seq: i16,
}

#[derive(Debug, FromRow)]
pub struct SimpleArticleEntityFromSQLx {
    pub id: String,
    pub title: String,
    pub language: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub seq: i16,
}


impl From<SimpleArticleEntityFromSQLx> for SimpleArticleEntity {
 fn from(value: SimpleArticleEntityFromSQLx) -> Self {
     Self {
         id: value.id.trim().to_owned(),
         title: value.title.trim().to_owned(),
         language: value.language.trim().to_owned(),
         created_at: value.created_at.timestamp_millis(),
         seq: value.seq,
     }
 }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleMemberEntity {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub seq: i16,
}

impl SimpleMemberEntity {
    pub fn new(id: String, name: String, avatar: Option<String>, seq: i16) -> Self {
        Self {
            id,
            name,
            avatar,
            seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct SimpleMemberEntityFromSQLx {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub seq: i16,
}

impl From<SimpleMemberEntityFromSQLx> for SimpleMemberEntity {
    fn from(value: SimpleMemberEntityFromSQLx) -> Self {
        Self {
            id: value.id.to_owned(),
            name: value.name.to_owned(),
            avatar: value.avatar,
            seq: value.seq,
        }
    }
}

#[derive(Debug)]
pub enum Pagination {
    All,
    Single,
    Page(Page),
}

#[derive(Debug)]
pub struct Page {
    pub page: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct UserID(uuid::Uuid);

impl TryFrom<String> for UserID {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match uuid::Uuid::try_parse(value.as_str()) {
            Ok(id) => Ok(UserID(id)),
            Err(_) => Err(()),
        }
    }
}

impl From<uuid::Uuid> for UserID {
    fn from(value: uuid::Uuid) -> Self {
        UserID(value)
    }
}

impl std::fmt::Display for UserID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
