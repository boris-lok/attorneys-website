use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::users::change_password;
use crate::domain::users::entity::UserID;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use secrecy::SecretBox;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ChangePasswordRequest {
    new_password: String,
}

pub async fn change_password(
    claims: Claims,
    state: State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<ChangePasswordRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let user_id = UserID::try_from(claims.sub.clone()).map_err(|_| ApiError::BadRequest)?;

    let req = change_password::Request {
        user_id,
        new_password: SecretBox::new(Box::new(req.new_password)),
    };

    match change_password::execute(&state.user_uow(), state.session_store.clone(), req).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(change_password::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
