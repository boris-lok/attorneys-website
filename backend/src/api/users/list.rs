use crate::api::api_error::ApiError;
use crate::repositories::SqlxUserRepository;
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    State(state): State<AppState>,
) -> Result<Json<ListUsersResponse>, ApiError> {
    use crate::domain::users::list_users;
    let conn = &mut *state
        .pool
        .acquire()
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let user_repo = SqlxUserRepository::new(Arc::new(Mutex::new(conn)));

    match list_users::execute(Mutex::new(user_repo)).await {
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
        Err(list_users::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
