use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::cases::delete::{execute, Error};
use crate::domain::cases::entity::CaseID;
use crate::infrastructure::db::case_repo::PostgresCaseRepo;
use crate::startup::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::collections::HashMap;

pub async fn delete_case(
    _: Claims,
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<StatusCode, ApiError> {
    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let case_id = CaseID::try_from(id.clone()).map_err(|_| ApiError::BadRequest)?;

    let mut repo = PostgresCaseRepo::new(&state.pool)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let res = execute(&mut repo, &case_id).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
