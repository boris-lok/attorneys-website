use crate::api::api_error::ApiError;
use crate::domain::cases::entity::CaseID;
use crate::domain::entity::Claims;
use crate::infrastructure::db::case_repo::PostgresCaseRepo;
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateCaseRequest {
    name: String,
    estimated_minutes: i32,
    billing_cycle: i32,
    started_at: chrono::DateTime<chrono::Utc>,
    ended_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateCaseResponse {
    id: String,
}

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

    let mut repo = PostgresCaseRepo::from_pool(&state.pool);

    let resp = crate::domain::cases::create::execute(&mut repo, req).await;

    match resp {
        Ok(id) => Ok(Json(CreateCaseResponse { id: id.into() })),
        Err(crate::domain::cases::create::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
