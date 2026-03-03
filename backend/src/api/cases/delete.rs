use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::cases::delete::{execute, Error, Request};
use crate::repositories::SQLxCaseRepository;
use crate::startup::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn delete_case(
    _: Claims,
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<StatusCode, ApiError> {
    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let req = Request { id: id.to_string() };

    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let repo = SQLxCaseRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let res = execute(Arc::new(Mutex::new(repo)), req).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::InvalidID) => Err(ApiError::BadRequest),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
