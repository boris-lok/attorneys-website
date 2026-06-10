use crate::api::api_error::ApiError;
use crate::domain::cases::entity::CaseID;
use crate::domain::cases::settle::Error;
use crate::domain::entity::Claims;
use crate::infrastructure::db::case_repo::PostgresCaseRepo;
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

    let mut repo = PostgresCaseRepo::from_pool(&state.pool);

    let res = crate::domain::cases::settle::execute(&mut repo, &case_id).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
