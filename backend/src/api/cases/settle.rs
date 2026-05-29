use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::cases::settle::Error;
use crate::repositories::SQLxCaseRepository;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct SettleRequest {
    case_id: String,
}

pub async fn settle(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<SettleRequest>, ApiError>,
) -> Result<impl IntoResponse, ApiError> {
    let req = crate::domain::cases::settle::Request {
        case_id: req.case_id,
    };

    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let case_repo = SQLxCaseRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let res = crate::domain::cases::settle::execute(req, Arc::new(Mutex::new(case_repo))).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::InvalidCaseID) => Err(ApiError::BadRequest),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
