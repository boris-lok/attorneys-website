use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::repositories::SQLxCaseRepository;
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct CreateCaseRequest {
    name: String,
    estimated_minutes: i32,
    started_at: chrono::DateTime<chrono::Utc>,
    ended_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateCaseResponse {
    id: String,
}

pub async fn create_case(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateCaseRequest>, ApiError>,
) -> Result<Json<CreateCaseResponse>, ApiError> {
    let req = crate::domain::cases::create::Request {
        name: req.name,
        estimated_minutes: req.estimated_minutes,
        started_at: req.started_at,
        ended_at: req.ended_at,
    };

    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let repo = SQLxCaseRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let resp = crate::domain::cases::create::execute(req, Arc::new(Mutex::new(repo))).await;

    match resp {
        Ok(id) => Ok(Json(CreateCaseResponse { id })),
        Err(crate::domain::cases::create::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
