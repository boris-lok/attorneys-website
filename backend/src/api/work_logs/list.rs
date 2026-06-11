use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::cases::entity::CaseID;
use crate::domain::entity::Claims;
use crate::domain::work_logs::entity::WorkLog;
use crate::domain::work_logs::error::WorkLogError;
use crate::domain::work_logs::list::{execute, Request};
use axum::extract::Query;
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
    QueryExtractor(q): QueryExtractor,
    query: Query<ListWorkLogsRequest>,
) -> Result<Json<ListWorkLogsResponse>, ApiError> {
    let req = Request {
        case_id: CaseID::try_from(query.case_id.clone()).map_err(|_| ApiError::BadRequest)?,
        started_at: query.started_at,
        ended_at: query.ended_at,
        include_settled: true,
    };

    let res = execute(&q, req).await;

    match res {
        Ok(work_logs) => Ok(Json(ListWorkLogsResponse { work_logs })),
        Err(WorkLogError::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(WorkLogError::NotFound) => Err(ApiError::NotFound),
        Err(WorkLogError::PermissionDenied) => Err(ApiError::PermissionDenied),
    }
}
