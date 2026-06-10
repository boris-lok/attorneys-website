use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

pub async fn logout(_: Claims, jar: CookieJar) -> Result<impl IntoResponse, ApiError> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::seconds(0));

    let jar = jar.remove(cookie);

    Ok((jar, StatusCode::OK))
}
