use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::{Resource, ServiceData};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use ulid::Ulid;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateServiceRequest {
    title: String,
    data: String,
    icon: String,
    language: String,
    seq: i32,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreateServiceResponse {
    id: String,
}

pub async fn create_service(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateServiceRequest>, ApiError>,
) -> Result<Json<CreateServiceResponse>, ApiError> {
    let id = Ulid::new().to_string();

    let request = crate::domain::resources::create::Request {
        id,
        data: Resource::Service(ServiceData::new(req.title, req.data, req.icon)),
        language: req.language,
        seq: req.seq,
    };

    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    match crate::domain::resources::create::execute(uow, request).await {
        Ok(id) => Ok(Json(CreateServiceResponse { id: id.to_string() })),
        Err(crate::domain::resources::create::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::create::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
