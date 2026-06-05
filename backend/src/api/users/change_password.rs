use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::users::entity::UserID;
use crate::infrastructure::db::connection::{PostgresRepo, UserRepo};
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use redis::TypedCommands;
use secrecy::SecretBox;
use serde::Deserialize;
use std::sync::Arc;

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
    let mut repo = PostgresRepo::<UserRepo>::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let user_id = UserID::try_from(claims.sub.clone()).map_err(|_| ApiError::BadRequest)?;

    let req = crate::domain::users::change_password::Request {
        user_id,
        new_password: SecretBox::new(Box::new(req.new_password)),
    };

    match crate::domain::users::change_password::execute(&mut repo, req).await {
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
