use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::UserID;
use crate::domain::work_logs::update::{execute, Error, Request};
use crate::repositories::SqlxWorkLogsRepository;
use crate::startup::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn delete_work_log(
    claims: Claims,
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<StatusCode, ApiError> {
    let user_id = claims.sub;
    let user_id = UserID::try_from(user_id).map_err(|_| ApiError::BadRequest)?;

    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let req = Request {
        id: id.to_string(),
        user_id,
        status: None,
        description: None,
        started_at: None,
        ended_at: None,
        deleted_at: Some(chrono::Utc::now()),
        force: false,
    };

    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let repo = SqlxWorkLogsRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let res = execute(Arc::new(Mutex::new(repo)), req).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(Error::InvalidID) => Err(ApiError::BadRequest),
        Err(Error::InvalidStatus(_)) => Err(ApiError::BadRequest),
        Err(Error::PermissionDenied) => Err(ApiError::PermissionDenied),
        Err(Error::NotFound) => Err(ApiError::NotFound),
    }
}
