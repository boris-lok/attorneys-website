use crate::api::api_error::ApiError;
use crate::domain::cases::delete::execute;
use crate::domain::cases::entity::CaseID;
use crate::domain::entity::Claims;
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

    execute(&state.case_uow(), &case_id).await?;

    Ok(StatusCode::OK)
}
