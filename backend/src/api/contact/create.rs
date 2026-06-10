use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::resources::create;
use crate::domain::resources::entity::{ContactData, Language, Resource};
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateContactRequest {
    data: serde_json::Value,
    language: String,
    seq: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateContactResponse {
    id: String,
}

pub async fn create_contact(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateContactRequest>, ApiError>,
) -> Result<Json<CreateContactResponse>, ApiError> {
    let resource = Resource::Contact(ContactData { data: req.data });

    let (kind, data) = resource
        .into_typed_content()
        .map_err(|_| ApiError::BadRequest)?;

    let req = create::Request {
        kind,
        data,
        seq: req.seq,
        language: Language::try_from(req.language).map_err(|_| ApiError::BadRequest)?,
    };

    let service = state.resource_uow();

    match create::execute(&service, req).await {
        Ok(id) => Ok(Json(CreateContactResponse { id: id.to_string() })),
        Err(create::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
