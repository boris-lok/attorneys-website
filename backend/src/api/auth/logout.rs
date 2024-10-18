use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use axum::http::StatusCode;
use axum::Extension;
use redis::Commands;
use std::sync::Arc;

pub async fn logout(
    claims: Claims,
    Extension(redis_client): Extension<Arc<redis::Client>>,
) -> Result<StatusCode, ApiError> {
    let user_id = claims.sub;

    let mut conn = redis_client
        .get_connection()
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let () = conn
        .del(&user_id)
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(StatusCode::OK)
}
