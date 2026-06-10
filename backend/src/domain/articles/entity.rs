use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct ArticleData {
    pub category_id: Option<String>,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleEntity {
    pub id: String,
    pub language: String,
    pub data: ArticleData,
    pub seq: i16,
}

#[derive(Debug, Serialize, Validate, Deserialize, Clone, Eq, PartialEq)]
pub struct CategoryData {
    pub icon: Option<String>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryEntity {
    pub id: String,
    pub language: String,
    pub data: CategoryData,
    pub seq: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleArticleEntity {
    pub id: String,
    pub title: String,
    pub language: String,
    pub created_at: i64,
    pub seq: i16,
}
