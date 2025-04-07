use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::{ContactData, Resource};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use ulid::Ulid;

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
    let id = Ulid::new().to_string();

    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let req = crate::domain::resources::create::Request {
        id: id.clone(),
        data: Resource::Contact(ContactData::new(req.data)),
        language: req.language,
        seq: req.seq,
    };

    match crate::domain::resources::create::execute(uow, req).await {
        Ok(id) => Ok(Json(CreateContactResponse { id: id.to_string() })),
        Err(crate::domain::resources::create::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::create::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
