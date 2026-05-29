use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::work_logs::list::{execute, Error, Request};
use crate::repositories::{SqlxWorkLogsRepository, WorkLog};
use crate::startup::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct ListWorkLogsRequest {
    case_id: String,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    ended_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ListWorkLogsResponse {
    work_logs: Vec<WorkLog>,
}

pub async fn list_work_logs(
    _: Claims,
    State(state): State<AppState>,
    query: Query<ListWorkLogsRequest>,
) -> Result<Json<ListWorkLogsResponse>, ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let repo = SqlxWorkLogsRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let req = Request {
        case_id: query.case_id.clone(),
        started_at: query.started_at,
        ended_at: query.ended_at,
        settled_at: None,
    };

    let res = execute(Arc::new(Mutex::new(repo)), req).await;

    match res {
        Ok(work_logs) => Ok(Json(ListWorkLogsResponse { work_logs })),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(Error::InvalidCaseID) => Err(ApiError::BadRequest),
    }
}
