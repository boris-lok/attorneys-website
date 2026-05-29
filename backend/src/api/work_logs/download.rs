use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::work_logs::list::{execute, Error, Request};
use crate::repositories::SqlxWorkLogsRepository;
use crate::startup::AppState;
use axum::extract::{Query, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct DownloadWorkLogsRequest {
    case_id: String,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    ended_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn download(
    _: Claims,
    State(state): State<AppState>,
    query: Query<DownloadWorkLogsRequest>,
) -> Result<impl IntoResponse, ApiError> {
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
        include_settled: false,
    };

    let res = execute(Arc::new(Mutex::new(repo)), req).await;

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
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(Error::InvalidCaseID) => Err(ApiError::BadRequest),
    }
}
