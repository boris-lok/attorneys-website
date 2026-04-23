use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::UserID;
use crate::repositories::SqlxWorkLogsRepository;
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateWorkLogRequest {
    case_id: String,
    started_at: chrono::DateTime<chrono::Utc>,
    duration: i64,
    description: String,
    collaborators: Option<Vec<String>>,
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
    let creator_id = Uuid::parse_str(claims.sub.as_str()).unwrap();
    let creator_id = UserID::from(creator_id);
    let case_id = Uuid::parse_str(req.case_id.as_str()).map_err(|_| ApiError::BadRequest)?;
    let collaborators = req.collaborators.map(|c| {
        c.iter()
            .filter_map(|id| {
                let id = Uuid::parse_str(id.as_str());
                id.map(UserID::from).ok()
            })
            .collect::<Vec<_>>()
    });

    let req = crate::domain::work_logs::create::Request {
        creator_id,
        case_id,
        started_at: req.started_at,
        duration: chrono::Duration::minutes(req.duration),
        description: req.description,
        collaborators,
    };

    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let repo = SqlxWorkLogsRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let res = crate::domain::work_logs::create::execute(Arc::new(Mutex::new(repo)), req).await;

    match res {
        Ok(id) => Ok(Json(CreateWorkLogResponse { id: id.to_string() })),
        Err(crate::domain::work_logs::create::Error::InvalidCaseID) => Err(ApiError::BadRequest),
        Err(crate::domain::work_logs::create::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
