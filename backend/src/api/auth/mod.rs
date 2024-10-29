use crate::api::api_error::ApiError;
use crate::startup::AppState;
use anyhow::Context;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use chrono::Utc;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod login;

pub mod logout;

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
