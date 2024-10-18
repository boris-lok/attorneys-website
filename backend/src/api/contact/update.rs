use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::api::update_resource_handler;
use crate::domain::entities::{ContactData, Resource};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct UpdateContactRequest {
    id: String,
    address: String,
    phone: String,
    email: String,
    language: String,
}

pub async fn update_contact(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateContactRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let req = crate::domain::resources::update::Request {
        id: req.id,
        data: Resource::Contact(ContactData::new(req.address, req.phone, req.email)),
        language: req.language,
    };

    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    update_resource_handler(uow, req).await
}
