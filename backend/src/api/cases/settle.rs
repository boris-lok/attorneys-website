use crate::api::api_error::ApiError;
use crate::domain::cases::entity::CaseID;
use crate::domain::cases::settle;
use crate::domain::entity::Claims;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SettleRequest {
    case_id: String,
}

pub async fn settle(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<SettleRequest>, ApiError>,
) -> Result<impl IntoResponse, ApiError> {
    let case_id = CaseID::try_from(req.case_id).map_err(|_| ApiError::BadRequest)?;

    settle::execute(&state.case_uow(), &case_id).await?;

    Ok(StatusCode::OK)
}
