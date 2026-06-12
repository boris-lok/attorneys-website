use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::users::list;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SimpleUser {
    pub id: String,
    pub nickname: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ListUsersResponse {
    users: Vec<SimpleUser>,
}
pub async fn list_users(
    QueryExtractor(query): QueryExtractor,
) -> Result<Json<ListUsersResponse>, ApiError> {
    match list::execute(&query).await {
        Ok(users) => Ok(Json(ListUsersResponse {
            users: users
                .into_iter()
                .map(|u| SimpleUser {
                    id: u.id.to_string(),
                    nickname: u.nickname,
                    roles: u.roles,
                })
                .collect(),
        })),
        Err(list::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
