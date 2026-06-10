use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::startup::AppState;
use anyhow::Context;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::Utc;
use redis::Commands;
use std::sync::Arc;

impl FromRequestParts<AppState> for Claims {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let now = Utc::now().timestamp() as usize;
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

        let jar = axum_extra::extract::CookieJar::from_request_parts(parts, &state)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        let token = jar.get("token").ok_or(ApiError::MissingBearer)?.value();

        let token_data = jsonwebtoken::decode::<Claims>(
            token,
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
