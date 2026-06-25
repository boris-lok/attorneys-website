use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::entity::WorkLogMappingStatus;
use crate::domain::work_logs::update::{execute, Request};
use crate::startup::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub description: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<i64>,
    pub status: Option<String>,
}

pub async fn update_work_log(
    claims: Claims,
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let id = Uuid::parse_str(id).map_err(|_| ApiError::BadRequest)?;

    let ended_at = req
        .started_at
        .map(|started_at| started_at + chrono::Duration::minutes(req.duration.unwrap_or(0)));

    let req = Request {
        id,
        user_id: UserID::try_from(claims.sub).map_err(|_| ApiError::BadRequest)?,
        description: req.description,
        started_at: req.started_at,
        ended_at,
        force: false,
        status: req
            .status
            .map(WorkLogMappingStatus::try_from)
            .transpose()
            .map_err(|_| ApiError::BadRequest)?,
    };

    execute(&state.work_log_uow(), req).await?;

    Ok(StatusCode::OK)
}
