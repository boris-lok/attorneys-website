use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error("Internal Server Error")]
    InternalServerError(String),
    #[error("Bad Request")]
    BadRequest,
    #[error("Not found")]
    NotFound,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("missing bearer token")]
    MissingBearer,
    #[error("credentials is expired")]
    ExpiredCredentials,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::JsonExtractorRejection(json_rejection) => (
                json_rejection.status(),
                format!("Json parsing error: {}", json_rejection.body_text()),
            ),
            ApiError::InternalServerError(reason) => (StatusCode::INTERNAL_SERVER_ERROR, reason),
            ApiError::BadRequest => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::InvalidCredentials => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::MissingBearer => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::ExpiredCredentials => (StatusCode::FORBIDDEN, self.to_string()),
        };

        let payload = json!({
            "message": message,
        });

        (status, Json(payload)).into_response()
    }
}
