use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::session::store::SessionStore;
use crate::startup::AppState;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use jsonwebtoken::DecodingKey;
use std::sync::Arc;

const SESSION_COOKIE_NAME: &str = "token";

impl FromRequestParts<AppState> for Claims {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Try Bearer token first (native app)
        let token = extract_bearer_token(parts);

        // 2. Fall back to cookie (web)
        let token = match token {
            None => {
                let jar = axum_extra::extract::CookieJar::from_request_parts(parts, &state)
                    .await
                    .map_err(|_| ApiError::Unauthorized)?;

                jar.get(SESSION_COOKIE_NAME)
                    .ok_or(ApiError::Unauthorized)?
                    .value()
                    .to_string()
            }
            Some(token) => token,
        };

        let claims =
            verify_token(&token, &state.jwt_decoding_key, state.session_store.clone()).await?;

        Ok(claims)
    }
}

fn extract_bearer_token(parts: &Parts) -> Option<String> {
    parts
        .headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|v| v.to_string())
}

async fn verify_token(
    token: &str,
    decoding_key: &DecodingKey,
    session: Arc<dyn SessionStore + Send + Sync>,
) -> Result<Claims, ApiError> {
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);

    let data =
        jsonwebtoken::decode::<Claims>(token, decoding_key, &validation).map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::ExpiredCredentials,
                _ => ApiError::Unauthorized,
            }
        })?;

    let res = session
        .get_session(&data.claims.sub)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    if res.is_none() {
        return Err(ApiError::ExpiredCredentials);
    }

    Ok(data.claims)
}
