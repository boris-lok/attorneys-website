use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::users::entity::UserID;
use crate::domain::work_logs::error::WorkLogError;
use crate::domain::work_logs::update::{execute, Request};
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub id: String,
    pub description: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<i64>,
}

pub async fn update_work_log(
    claims: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let ended_at = req
        .started_at
        .map(|started_at| started_at + chrono::Duration::minutes(req.duration.unwrap_or(0)));

    let req = Request {
        id: Uuid::parse_str(&req.id).map_err(|_| ApiError::BadRequest)?,
        user_id: UserID::try_from(claims.sub).map_err(|_| ApiError::BadRequest)?,
        description: req.description,
        started_at: req.started_at,
        ended_at,
        force: false,
    };

    let res = execute(&state.work_log_uow(), req).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(WorkLogError::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(WorkLogError::PermissionDenied) => Err(ApiError::PermissionDenied),
        Err(WorkLogError::NotFound) => Err(ApiError::NotFound),
    }
}
