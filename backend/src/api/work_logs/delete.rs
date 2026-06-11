use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::users::entity::UserID;
use crate::domain::work_logs::delete::{execute, Request};
use crate::domain::work_logs::error::WorkLogError;
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

    let res = execute(&state.work_log_uow(), req).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(WorkLogError::Unknown(e)) => Err(ApiError::InternalServerError(e)),
        Err(WorkLogError::NotFound) => Err(ApiError::NotFound),
        Err(WorkLogError::PermissionDenied) => Err(ApiError::PermissionDenied),
    }
}
