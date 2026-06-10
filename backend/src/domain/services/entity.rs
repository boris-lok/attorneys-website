use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct ServiceData {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub data: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceEntity {
    pub id: String,
    pub language: String,
    pub data: ServiceData,
    pub seq: i16,
}
