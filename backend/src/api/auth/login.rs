use crate::api::api_error::ApiError;
use crate::domain::users::authentication::{validate_credentials, Credentials, Error};
use crate::repositories::SqlxUserRepository;
use crate::startup::AppState;
use anyhow::Context;
use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::request::Parts;
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

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let now = Utc::now().timestamp() as usize;
        let state = AppState::from_ref(state);
        let redis_client = parts.extensions.get::<Arc<redis::Client>>();

        if redis_client.is_none() {
            return Err(ApiError::InternalServerError(
                "can't get redis client from extensions".to_string(),
            ));
        }

        let mut redis_connection = redis_client
            .unwrap()
            .get_connection()
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        let bearer = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split_once(' '))
            .context("Missing Authorization header")
            .map_err(|_| ApiError::MissingBearer)?
            .1;

        let token_data = jsonwebtoken::decode::<Claims>(
            bearer,
            &state.jwt_decoding_key,
            &jsonwebtoken::Validation::default(),
        )
        .context("Failed to decode jwt")
        .map_err(|_| ApiError::InvalidCredentials)?;

        let exp_from_session: Option<usize> = redis_connection
            .get(&token_data.claims.sub)
            .context("Failed to get expired date by token")
            .map_err(|_| ApiError::InvalidCredentials)?;

        if exp_from_session.is_none() || exp_from_session.unwrap() < now {
            return Err(ApiError::ExpiredCredentials)?;
        }

        Ok(token_data.claims)
    }
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
