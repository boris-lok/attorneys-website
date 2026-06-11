use crate::api::api_error::ApiError;
use crate::domain::auth;
use crate::domain::entity::Claims;
use crate::domain::users::entity::UserID;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

pub async fn logout(
    c: Claims,
    jar: CookieJar,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = UserID::try_from(c.sub.clone()).map_err(|_| ApiError::BadRequest)?;

    auth::logout::execute(state.session_store.clone(), &user_id)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::seconds(0));

    let jar = jar.remove(cookie);

    Ok((jar, StatusCode::OK))
}
