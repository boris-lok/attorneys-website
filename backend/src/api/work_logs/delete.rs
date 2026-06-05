use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::UserID;
use crate::domain::work_logs::delete::{execute, Error, Request};
use crate::infrastructure::db::connection::{PostgresRepo, WorkLogRepo};
use crate::startup::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn delete_work_log(
    claims: Claims,
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<StatusCode, ApiError> {
    let user_id = UserID::try_from(claims.sub).map_err(|_| ApiError::BadRequest)?;
    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let id = Uuid::parse_str(id).map_err(|_| ApiError::BadRequest)?;
    let req = Request {
        id,
        user_id,
        force: false,
    };

    let mut repo = PostgresRepo::<WorkLogRepo>::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let res = execute(&mut repo, req).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(Error::InvalidID) => Err(ApiError::BadRequest),
        Err(Error::NotFound) => Err(ApiError::NotFound),
        Err(Error::PermissionDenied) => Err(ApiError::PermissionDenied),
    }
}
