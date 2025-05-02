use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::UserID;
use crate::repositories::{Connection, SqlxUserRepository};
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use secrecy::SecretBox;
use serde::Deserialize;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub(crate) struct ChangePasswordRequest {
    new_password: String,
}

pub async fn change_password(
    claims: Claims,
    state: State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<ChangePasswordRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| ApiError::BadRequest)?;
    let user_id = UserID::from(user_id);

    let req = crate::domain::users::change_password::Request {
        user_id,
        new_password: SecretBox::new(Box::new(req.new_password)),
    };
    let user_repo = SqlxUserRepository::new(Connection::Pool(state.pool.clone()));

    match crate::domain::users::change_password::execute(req, Mutex::new(user_repo)).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(crate::domain::users::change_password::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
