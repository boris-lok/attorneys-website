use crate::api::api_error::ApiError;
use crate::domain::cases::entity::CaseID;
use crate::domain::cases::settle;
use crate::domain::entity::Claims;
use crate::startup::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;


pub async fn settle(
    _: Claims,
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let case_id = CaseID::try_from(id.clone()).map_err(|_| ApiError::BadRequest)?;

    settle::execute(&state.case_uow(), &case_id).await?;

    Ok(StatusCode::OK)
}
