use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::users::authentication::{validate_credentials, Credentials, Error};
use crate::repositories::SqlxUserRepository;
use crate::startup::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use chrono::{Duration, Utc};
use redis::Commands;
use secrecy::SecretBox;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    user_id: String,
    username: String,
    token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Extension(redis_client): Extension<Arc<redis::Client>>,
    WithRejection(Json(req), _): WithRejection<Json<LoginRequest>, ApiError>,
) -> Result<Json<LoginResponse>, ApiError> {
    let tx = state
        .pool
        .begin()
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let tx = Arc::new(Mutex::new(tx));
    let user_repo = SqlxUserRepository::new(Arc::downgrade(&tx));

    let credentials = Credentials {
        username: req.username.clone(),
        password: SecretBox::new(Box::new(req.password)),
    };

    let res = validate_credentials(credentials, Mutex::new(user_repo)).await;

    match res {
        Ok(id) => {
            let user_id = id.to_string();
            let exp = Utc::now() + Duration::days(15);

            let mut redis_connection = redis_client
                .get_connection()
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            let () = redis_connection
                .set(&user_id, exp.timestamp() as usize)
                .context("Failed to set a user to session storage.")
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            let claims = Claims {
                sub: user_id.clone(),
                exp: exp.timestamp() as usize,
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &state.jwt_encoding_key,
            )
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            Ok(Json(LoginResponse {
                user_id,
                username: req.username,
                token,
            }))
        }
        Err(Error::InvalidCredentials) => Err(ApiError::InvalidCredentials),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
