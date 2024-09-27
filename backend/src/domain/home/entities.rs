// use serde::{Deserialize, Serialize};
// use std::fmt::Display;
//
// #[derive(Eq, PartialEq, Clone, Debug)]
// pub struct HomeID(String);
// impl TryFrom<String> for HomeID {
//     type Error = ();
//
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         match value.trim().is_empty() {
//             true => Err(()),
//             false => Ok(HomeID(value)),
//         }
//     }
// }
//
// impl Display for HomeID {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0.clone())
//     }
// }
//
// impl HomeID {
//     pub fn as_str(&self) -> &str {
//         &self.0
//     }
// }
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct HomeData {
//     pub(crate) data: String,
// }
//
// impl TryFrom<String> for HomeData {
//     type Error = ();
//
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         match value.trim().is_empty() {
//             true => Err(()),
//             false => Ok(HomeData { data: value }),
//         }
//     }
// }
//
// #[derive(Debug, Serialize, sqlx::FromRow)]
// pub struct Home {
//     pub home_id: String,
//     pub content: String,
// }
