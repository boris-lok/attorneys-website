use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::api::work_logs::entity::APIWorkLogFilters;
use crate::domain::cases::entity::CaseID;
use crate::domain::entity::Claims;
use crate::domain::work_logs::error::WorkLogError;
use crate::domain::work_logs::list::{execute, Request};
use axum::extract::Query;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DownloadWorkLogsRequest {
    case_id: String,
    #[serde(flatten)]
    filters: APIWorkLogFilters,
}

pub async fn download(
    _: Claims,
    QueryExtractor(q): QueryExtractor,
    query: Query<DownloadWorkLogsRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let req = Request {
        case_id: CaseID::try_from(query.case_id.clone()).map_err(|_| ApiError::BadRequest)?,
        filters: query.filters.clone().try_into()?,
    };

    let res = execute(&q, req).await;

    match res {
        Ok(work_logs) => {
            let data = tokio::task::spawn_blocking(move || {
                let mut workbook =
                    crate::domain::work_logs::generate_worksheet::generate(work_logs)
                        .map_err(|_| "Failed to generate workbook")?;

                workbook
                    .save_to_buffer()
                    .map_err(|_| "Failed to save workbook")
            })
            .await
            .map_err(|e| ApiError::InternalServerError(format!("Task join error: {}", e)))?
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

            Ok((
                StatusCode::OK,
                [(
                    header::CONTENT_TYPE,
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                )],
                data,
            ))
        }
        Err(WorkLogError::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(WorkLogError::NotFound) => Err(ApiError::NotFound),
        Err(WorkLogError::PermissionDenied) => Err(ApiError::PermissionDenied),
    }
}
