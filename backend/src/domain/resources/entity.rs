use std::fmt::Display;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResourceID {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
