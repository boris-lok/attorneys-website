use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::work_logs::update_status::{execute, Error, Request};
use crate::repositories::SqlxWorkLogsRepository;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub id: String,
    pub status: String,
}

pub async fn update_work_log_status(
    claims: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateStatusRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let user_id = claims.sub;

    let req = Request {
        id: req.id,
        user_id,
        status: req.status,
    };

    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let repo = SqlxWorkLogsRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let res = execute(Arc::new(Mutex::new(repo)), req).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::InvalidID) => Err(ApiError::BadRequest),
        Err(Error::NotFound) => Err(ApiError::NotFound),
        Err(Error::InvalidStatus) => Err(ApiError::BadRequest),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
