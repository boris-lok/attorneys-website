pub enum Language {
    TW,
    EN,
}

#[derive(Eq, PartialEq, Clone)]
pub struct MemberID(pub(crate) String);

impl TryFrom<String> for MemberID {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(MemberID(value))
        }
    }
}

pub struct MemberName(pub(crate) String);

impl TryFrom<String> for MemberName {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(MemberName(value))
        }
    }
}

impl TryFrom<String> for Language {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "en" => Ok(Language::EN),
            "tw" => Ok(Language::TW),
            _ => Err(()),
        }
    }
}

pub struct Member {
    pub(crate) member_id: MemberID,
    pub(crate) member_name: MemberName,
    pub(crate) default_language: Language,
}

impl Member {
    pub fn new(member_id: MemberID, member_name: MemberName, default_language: Language) -> Self {
        Member {
            member_id,
            member_name,
            default_language,
        }
    }
}
