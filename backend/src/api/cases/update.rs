use crate::api::api_error::ApiError;
use crate::domain::cases::entity::CaseID;
use crate::domain::cases::update::execute;
use crate::domain::entity::Claims;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateCaseRequest {
    pub id: String,
    pub name: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub estimated_minutes: Option<i32>,
    pub billing_cycle: Option<i32>,
}

pub async fn update_case(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateCaseRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let req = crate::domain::cases::entity::UpdateCaseRequest {
        id: CaseID::try_from(req.id).map_err(|_| ApiError::BadRequest)?,
        name: req.name,
        started_at: req.started_at,
        ended_at: req.ended_at,
        estimated_minutes: req.estimated_minutes,
        billing_cycle: req.billing_cycle,
    };

    execute(&state.case_uow(), req).await?;

    Ok(StatusCode::OK)
}
