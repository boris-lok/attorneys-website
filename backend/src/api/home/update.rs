use crate::api::api_error::ApiError;
use crate::api::resources::update::execute_update;
use crate::domain::entity::Claims;
use crate::domain::resources::entity::{HomeData, Resource};
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateServiceRequest {
    id: String,
    data: String,
    language: String,
    seq: i32,
}

pub async fn update_home(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateServiceRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let resource = Resource::Home(HomeData { data: req.data });

    execute_update(&state, req.id, req.seq, req.language, resource).await
}
