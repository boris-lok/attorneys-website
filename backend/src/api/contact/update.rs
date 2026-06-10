use crate::api::api_error::ApiError;
use crate::api::resources::update::execute_update;
use crate::domain::entity::Claims;
use crate::domain::resources::entity::{ContactData, Resource};
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateContactRequest {
    id: String,
    data: serde_json::Value,
    language: String,
    seq: i32,
}

pub async fn update_contact(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateContactRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let resource = Resource::Contact(ContactData { data: req.data });

    execute_update(&state, req.id, req.seq, req.language, resource).await
}
