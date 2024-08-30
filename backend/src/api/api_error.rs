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
    InternalServerError,
    #[error("Member already exists")]
    MemberAlreadyExists,
    #[error("Bad Request")]
    BadRequest,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::JsonExtractorRejection(json_rejection) => (
                json_rejection.status(),
                format!("Json parsing error: {}", json_rejection.body_text()),
            ),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::MemberAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            ApiError::BadRequest => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let payload = json!({
            "message": message,
        });

        (status, Json(payload)).into_response()
    }
}
