use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub roles: Vec<String>,
    pub nickname: String,
}
