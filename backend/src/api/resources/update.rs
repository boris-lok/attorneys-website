use crate::api::api_error::ApiError;
use crate::domain::resources::entity::{Language, Resource, ResourceID};
use crate::domain::resources::update;
use crate::startup::AppState;
use axum::http::StatusCode;

// Shared helper
pub async fn execute_update(
    state: &AppState,
    id: String,
    seq: i32,
    language: String,
    resource: Resource,
) -> Result<StatusCode, ApiError> {
    let (kind, data) = resource
        .into_typed_content()
        .map_err(|_| ApiError::BadRequest)?;

    let req = update::Request {
        id: ResourceID::try_from(id).map_err(|_| ApiError::BadRequest)?,
        kind,
        data,
        seq,
        language: Language::try_from(language).map_err(|_| ApiError::BadRequest)?,
    };

    let uow = state.resource_uow();

    match update::execute(&uow, req).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(update::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
