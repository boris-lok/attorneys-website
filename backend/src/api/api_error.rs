use crate::domain::cases::error::CaseError;
use crate::domain::work_logs::error::WorkLogError;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use utoipa::ToSchema;

#[derive(thiserror::Error, Debug, ToSchema)]
pub enum ApiError {
    #[error(transparent)]
    #[schema(value_type = String)]
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
    #[error("permission denied")]
    PermissionDenied,
    #[error("unauthorized")]
    Unauthorized,
    #[error("{message}")]
    Custom {
        #[schema(value_type = u16)]
        status_code: StatusCode,
        message: String,
    },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::JsonExtractorRejection(json_rejection) => (
                json_rejection.status(),
                format!("Json parsing error: {}", json_rejection.body_text()),
            ),
            ApiError::InternalServerError(reason) => {
                tracing::error!("Internal server error: {}", reason);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            ApiError::BadRequest => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::InvalidCredentials => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::MissingBearer => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::ExpiredCredentials => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::PermissionDenied => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::Custom {
                status_code,
                message,
            } => (status_code, message),
        };

        let payload = json!({
            "message": message,
        });

        (status, Json(payload)).into_response()
    }
}

impl From<WorkLogError> for ApiError {
    fn from(value: WorkLogError) -> Self {
        match value {
            WorkLogError::NotFound => ApiError::Custom {
                status_code: StatusCode::NOT_FOUND,
                message: "work log not found".to_string(),
            },
            WorkLogError::PermissionDenied => ApiError::Custom {
                status_code: StatusCode::FORBIDDEN,
                message: "you don't have permission to operate this work log".to_string(),
            },
            WorkLogError::Unknown(e) => ApiError::InternalServerError(e),
            WorkLogError::CaseIsClosed => ApiError::Custom {
                status_code: StatusCode::FORBIDDEN,
                message: "case is closed".to_string(),
            },
            WorkLogError::CaseNotFound => ApiError::Custom {
                status_code: StatusCode::NOT_FOUND,
                message: "case not found".to_string(),
            },
        }
    }
}

impl From<CaseError> for ApiError {
    fn from(value: CaseError) -> Self {
        match value {
            CaseError::NotFound => ApiError::Custom {
                status_code: StatusCode::NOT_FOUND,
                message: "case not found".to_string(),
            },
            CaseError::CaseIsClosed => ApiError::Custom {
                status_code: StatusCode::FORBIDDEN,
                message: "case is closed".to_string(),
            },
            CaseError::Unknown(e) => ApiError::InternalServerError(e),
        }
    }
}
