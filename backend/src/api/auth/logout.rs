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
    let jar = jar.remove(
        Cookie::build(("token", ""))
            .path("/")
            .max_age(time::Duration::seconds(0)),
    );

    let status = match UserID::try_from(c.sub.clone()) {
        Ok(user_id) => match auth::logout::execute(state.session_store.clone(), &user_id).await {
            Ok(()) => StatusCode::OK,
            Err(e) => {
                tracing::error!("session invalidation failed: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
        Err(_) => StatusCode::BAD_REQUEST,
    };
    Ok((jar, status))
}
