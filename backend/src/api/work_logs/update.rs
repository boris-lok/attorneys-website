use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::UserID;
use crate::domain::work_logs::update::{execute, Error, Request};
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
pub struct UpdateRequest {
    pub id: String,
    pub status: Option<String>,
    pub description: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<i64>,
}

pub async fn update_work_log(
    claims: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let user_id = claims.sub;
    let user_id = UserID::try_from(user_id).map_err(|_| ApiError::BadRequest)?;

    let ended_at = req
        .started_at
        .map(|started_at| started_at + chrono::Duration::minutes(req.duration.unwrap_or(0)));

    let req = Request {
        id: req.id,
        user_id,
        status: req.status,
        description: req.description,
        started_at: req.started_at,
        ended_at,
        force: false,
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
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(Error::InvalidID) => Err(ApiError::BadRequest),
        Err(Error::InvalidStatus(_)) => Err(ApiError::BadRequest),
        Err(Error::PermissionDenied) => Err(ApiError::PermissionDenied),
        Err(Error::NotFound) => Err(ApiError::NotFound),
    }
}
