use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MemberData {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MemberDetails {
    pub name: String,
    pub data: MemberData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub tags: Vec<String>,
    pub details: HashMap<String, MemberDetails>,
}
