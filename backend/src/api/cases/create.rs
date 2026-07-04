use crate::api::api_error::ApiError;
use crate::domain::cases::entity::CaseID;
use crate::domain::entity::Claims;
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCaseRequest {
    name: String,
    estimated_minutes: i32,
    billing_cycle: i32,
    started_at: chrono::DateTime<chrono::Utc>,
    ended_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateCaseResponse {
    id: String,
}

#[utoipa::path(
    post,
    path = "/cases",
    request_body = CreateCaseRequest,
    responses(
        (status = 200, description = "Case created", body = CreateCaseResponse),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal Server Error", body = ApiError)
    ),
)]
pub async fn create_case(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateCaseRequest>, ApiError>,
) -> Result<Json<CreateCaseResponse>, ApiError> {
    let req = crate::domain::cases::entity::CreateCaseRequest {
        id: CaseID::from(Uuid::new_v4()),
        name: req.name,
        estimated_minutes: req.estimated_minutes,
        billing_cycle: req.billing_cycle,
        started_at: req.started_at,
        ended_at: req.ended_at,
    };

    let resp = crate::domain::cases::create::execute(&state.case_uow(), req).await?;

    Ok(Json(CreateCaseResponse { id: resp.into() }))
}
