use crate::api::api_error::ApiError;
use crate::api::resources::update::execute_update;
use crate::domain::entity::Claims;
use crate::domain::resources::entity::Resource;
use crate::domain::services::entity::ServiceData;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateServiceRequest {
    id: String,
    title: String,
    data: String,
    icon: String,
    language: String,
    seq: i32,
}

pub async fn update_service(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateServiceRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let resource = Resource::Service(ServiceData {
        title: req.title,
        data: req.data,
        icon: req.icon,
    });

    execute_update(&state, req.id, req.seq, req.language, resource).await
}
