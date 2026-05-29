use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::UserID;
use crate::repositories::SqlxUserRepository;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use redis::TypedCommands;
use secrecy::SecretBox;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub(crate) struct ChangePasswordRequest {
    new_password: String,
}

pub async fn change_password(
    claims: Claims,
    state: State<AppState>,
    Extension(redis_client): Extension<Arc<redis::Client>>,
    WithRejection(Json(req), _): WithRejection<Json<ChangePasswordRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| ApiError::BadRequest)?;
    let user_id = UserID::from(user_id);

    let req = crate::domain::users::change_password::Request {
        user_id,
        new_password: SecretBox::new(Box::new(req.new_password)),
    };
    let user_repo = SqlxUserRepository::new(Arc::new(Mutex::new(conn.as_mut())));

    match crate::domain::users::change_password::execute(req, Mutex::new(user_repo)).await {
        Ok(_) => {
            let mut c = redis_client
                .get_connection()
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
            c.del(&claims.sub)
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            Ok(StatusCode::OK)
        }
        Err(crate::domain::users::change_password::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
