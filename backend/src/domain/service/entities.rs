use serde::Serialize;
use std::fmt::Display;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ServiceID(String);
impl TryFrom<String> for ServiceID {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim().is_empty() {
            true => Err(()),
            false => Ok(ServiceID(value)),
        }
    }
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl ServiceID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize)]
pub struct ServiceData {
    pub(crate) data: String,
}

impl TryFrom<String> for ServiceData {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim().is_empty() {
            true => Err(()),
            false => Ok(ServiceData { data: value }),
        }
    }
}
