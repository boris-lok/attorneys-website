use crate::api::api_error::ApiError;
use crate::domain::cases::entity::CaseID;
use crate::domain::entity::Claims;
use crate::domain::work_logs::entity::WorkLog;
use crate::domain::work_logs::list::{execute, Error, Request};
use crate::infrastructure::db::connection::{PostgresRepo, WorkLogRepo};
use crate::startup::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

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
    let mut repo = PostgresRepo::<WorkLogRepo>::from_pool(&state.pool);

    let req = Request {
        case_id: CaseID::try_from(query.case_id.clone()).map_err(|_| ApiError::BadRequest)?,
        started_at: query.started_at,
        ended_at: query.ended_at,
        include_settled: true,
    };

    let res = execute(&mut repo, req).await;

    match res {
        Ok(work_logs) => Ok(Json(ListWorkLogsResponse { work_logs })),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
