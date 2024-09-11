use crate::domain::create_member::Data;
use serde::Serialize;

#[derive(Debug)]
pub enum Language {
    ZH,
    EN,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct MemberID(pub(crate) String);

impl TryFrom<String> for MemberID {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.is_empty() {
            true => Err(()),
            false => Ok(MemberID(value)),
        }
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
#[derive(Debug, Serialize)]
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

#[derive(Debug)]
pub struct Member {
    pub(crate) member_id: MemberID,
}

impl Member {
    pub fn new(member_id: MemberID) -> Self {
        Member { member_id }
    }
}
#[derive(Debug, Clone)]
pub struct ContentID(pub(crate) String);

#[derive(Debug, Clone)]
pub struct ContentData(pub(crate) serde_json::Value);

impl TryFrom<MemberData> for ContentData {
    type Error = anyhow::Error;

    fn try_from(value: MemberData) -> anyhow::Result<Self> {
        let data = serde_json::value::to_value(&value)?;
        Ok(ContentData(data))
    }
}

#[derive(Debug, Serialize)]
pub struct AvatarData {
    pub(crate) large_image: String,
    pub(crate) small_image: String,
}

#[derive(Debug)]
pub struct AvatarJson(pub(crate) serde_json::Value);
