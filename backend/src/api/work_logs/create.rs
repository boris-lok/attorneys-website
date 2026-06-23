use crate::api::api_error::ApiError;
use crate::domain::cases::entity::CaseID;
use crate::domain::entity::Claims;
use crate::domain::users::entity::UserID;
use crate::domain::work_logs::error::WorkLogError;
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateWorkLogRequest {
    case_id: String,
    started_at: chrono::DateTime<chrono::Utc>,
    duration: i64,
    description: String,
    collaborator_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct CreateWorkLogResponse {
    id: String,
}

pub async fn create_work_log(
    claims: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateWorkLogRequest>, ApiError>,
) -> Result<Json<CreateWorkLogResponse>, ApiError> {
    let collaborator_ids = req
        .collaborator_ids
        .unwrap_or_default()
        .iter()
        .map(|id| {
            Uuid::parse_str(id)
                .map(UserID::from)
                .map_err(|_| ApiError::BadRequest)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let req = crate::domain::work_logs::create::Request {
        id: Uuid::new_v4(),
        user_id: Uuid::parse_str(&claims.sub).map_err(|_| ApiError::BadRequest)?,
        case_id: CaseID::from(Uuid::parse_str(&req.case_id).map_err(|_| ApiError::BadRequest)?),
        started_at: req.started_at,
        ended_at: req.started_at + chrono::Duration::minutes(req.duration),
        description: req.description,
        collaborator_ids,
    };

    let res = crate::domain::work_logs::create::execute(&state.work_log_uow(), req).await;

    match res {
        Ok(id) => Ok(Json(CreateWorkLogResponse { id: id.to_string() })),
        Err(WorkLogError::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(WorkLogError::NotFound) => Err(ApiError::NotFound),
        Err(WorkLogError::PermissionDenied) => Err(ApiError::PermissionDenied),
        Err(WorkLogError::CaseIsClosed) => Err(ApiError::Forbidden),
        Err(WorkLogError::CaseNotFound) => Err(ApiError::NotFound),
    }
}
