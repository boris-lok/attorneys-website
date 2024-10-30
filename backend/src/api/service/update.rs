use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::api::update_resource_handler;
use crate::domain::entities::{Resource, ServiceData};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateServiceRequest {
    id: String,
    title: String,
    data: String,
    language: String,
    seq: i32,
}

pub async fn update_service(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateServiceRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let req = crate::domain::resources::update::Request {
        id: req.id,
        data: Resource::Service(ServiceData::new(req.title, req.data)),
        language: req.language,
        seq: req.seq,
    };

    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    update_resource_handler(uow, req).await
}
