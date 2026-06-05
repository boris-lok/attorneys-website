use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct UserID(Uuid);

impl TryFrom<String> for UserID {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Uuid::try_parse(value.as_str()) {
            Ok(id) => Ok(UserID(id)),
            Err(_) => Err(format!("Invalid user id: {}", value)),
        }
    }
}

impl From<Uuid> for UserID {
    fn from(value: Uuid) -> Self {
        UserID(value)
    }
}

impl From<&UserID> for Uuid {
    fn from(value: &UserID) -> Self {
        value.0
    }
}

impl Display for UserID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct User {
    pub id: UserID,
    pub username: String,
    pub nickname: String,
    pub roles: Vec<String>,
}
