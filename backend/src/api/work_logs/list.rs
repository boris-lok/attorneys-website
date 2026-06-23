use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::api::work_logs::entity::APIWorkLogFilters;
use crate::domain::cases::entity::CaseID;
use crate::domain::entity::Claims;
use crate::domain::work_logs::entity::WorkLog;
use crate::domain::work_logs::list::{execute, Request};
use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListWorkLogsRequest {
    case_id: String,
    #[serde(flatten)]
    filters: APIWorkLogFilters,
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
        filters: query.filters.clone().try_into()?,
    };

    let res = execute(&q, req).await?;

    Ok(Json(ListWorkLogsResponse { work_logs: res }))
}
