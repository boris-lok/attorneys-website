use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::users::entity::UserID;
use crate::domain::work_log_mapping::entity::WorkLogMappingStatus;
use crate::domain::work_logs::update_status::{execute, Error, Request};
use crate::infrastructure::db::connection::{PostgresRepo, WorkLogMappingRepo, WorkLogRepo};
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use uuid::Uuid;

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
        id: Uuid::parse_str(&req.id).map_err(|_| ApiError::BadRequest)?,
        user_id: UserID::try_from(user_id).map_err(|_| ApiError::BadRequest)?,
        status: WorkLogMappingStatus::try_from(req.status).map_err(|_| ApiError::BadRequest)?,
    };

    let mut work_log_repo = PostgresRepo::<WorkLogRepo>::from_pool(&state.pool);
    let mut work_log_mapping_repo = PostgresRepo::<WorkLogMappingRepo>::from_pool(&state.pool);

    let res = execute(&mut work_log_repo, &mut work_log_mapping_repo, req).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::NotFound) => Err(ApiError::NotFound),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
