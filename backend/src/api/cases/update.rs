use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::cases::update::{execute, Error, Request};
use crate::repositories::SQLxCaseRepository;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct UpdateCaseRequest {
    pub id: String,
    pub name: Option<String>,
    pub estimated_minutes: Option<i32>,
}

pub async fn update_case(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateCaseRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let req = Request {
        id: req.id,
        name: req.name,
        estimated_minutes: req.estimated_minutes,
    };

    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let repo = SQLxCaseRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let res = execute(Arc::new(Mutex::new(repo)), req).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
