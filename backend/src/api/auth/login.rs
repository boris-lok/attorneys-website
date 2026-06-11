use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::auth;
use crate::domain::auth::login::Error;
use crate::domain::users::authentication::Credentials;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::{CookieJar, WithRejection};
use secrecy::SecretBox;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[axum_macros::debug_handler]
pub async fn login(
    QueryExtractor(query): QueryExtractor,
    State(state): State<AppState>,
    jar: CookieJar,
    WithRejection(Json(req), _): WithRejection<Json<LoginRequest>, ApiError>,
) -> Result<impl IntoResponse, ApiError> {
    let credentials = Credentials {
        username: req.username.clone(),
        password: SecretBox::new(Box::new(req.password)),
    };

    match auth::login::execute(
        &query,
        state.session_store.clone(),
        state.jwt_encoding_key.clone(),
        credentials,
    )
    .await
    {
        Ok(token) => {
            let ct = Cookie::build(("token", token))
                .path("/")
                .http_only(true)
                .secure(false)
                .same_site(SameSite::Lax);

            let jar = jar.add(ct);

            Ok((jar, StatusCode::OK))
        }
        Err(Error::InvalidCredentials) => Err(ApiError::InvalidCredentials),
        Err(Error::CreateJWTFailed) => Err(ApiError::InternalServerError(
            "Failed to create JWT".to_string(),
        )),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
