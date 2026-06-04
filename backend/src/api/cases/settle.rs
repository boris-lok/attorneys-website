use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::cases::entity::CaseID;
use crate::domain::cases::settle::Error;
use crate::infrastructure::db::case_repo::PostgresCaseRepo;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

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

    let repo = PostgresCaseRepo::new(&state.pool)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let res = crate::domain::cases::settle::execute(Arc::new(Mutex::new(repo)), &case_id).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
