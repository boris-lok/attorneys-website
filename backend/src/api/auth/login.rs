use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::users::authentication::{validate_credentials, Credentials, Error};
use crate::domain::users::repository::{UserRepository, UserRoleRepository};
use crate::infrastructure::db::connection::{PostgresRepo, UserRepo, UserRoleRepo};
use crate::startup::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::{CookieJar, WithRejection};
use chrono::{Duration, Utc};
use redis::Commands;
use secrecy::SecretBox;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[axum_macros::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Extension(redis_client): Extension<Arc<redis::Client>>,
    WithRejection(Json(req), _): WithRejection<Json<LoginRequest>, ApiError>,
) -> Result<impl IntoResponse, ApiError> {
    let mut repo = PostgresRepo::<UserRepo>::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let credentials = Credentials {
        username: req.username.clone(),
        password: SecretBox::new(Box::new(req.password)),
    };

    let res = validate_credentials(&mut repo, credentials).await;

    match res {
        Ok(id) => {
            let mut user_role_repo = PostgresRepo::<UserRoleRepo>::new(&state.pool)
                .await
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            let user_id = id.to_string();
            let exp = Utc::now() + Duration::days(30);

            let mut redis_connection = redis_client
                .get_connection()
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            let () = redis_connection
                .set(&user_id, exp.timestamp() as usize)
                .context("Failed to set a user to session storage.")
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            let roles = user_role_repo
                .get_user_roles(&id)
                .await
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
            let nickname = repo
                .get_user_nickname(&id)
                .await
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            let claims = Claims {
                sub: user_id.clone(),
                exp: exp.timestamp() as usize,
                roles: roles.clone(),
                nickname: nickname.clone(),
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &state.jwt_encoding_key,
            )
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            let ct = Cookie::build(("token", token))
                .path("/")
                .http_only(true)
                .secure(false)
                .same_site(SameSite::Lax);

            let jar = jar.add(ct);

            Ok((jar, StatusCode::OK))
        }
        Err(Error::InvalidCredentials) => Err(ApiError::InvalidCredentials),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
