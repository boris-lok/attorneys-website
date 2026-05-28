use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum_extra::extract::CookieJar;
use redis::Commands;
use std::sync::Arc;
use axum_extra::extract::cookie::Cookie;

pub async fn logout(
    claims: Claims,
    jar: CookieJar,
    Extension(redis_client): Extension<Arc<redis::Client>>,
) -> Result<impl IntoResponse, ApiError> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::seconds(0));

    let jar = jar.remove(cookie);

    Ok((jar, StatusCode::OK))
}
