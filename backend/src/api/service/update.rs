use crate::api::api_error::ApiError;
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
}

pub async fn update_service(
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateServiceRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let req = crate::domain::resources::update::Request {
        id: req.id,
        data: Resource::Service(ServiceData::new(req.title, req.data)),
        language: req.language,
    };

    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    match crate::domain::resources::update::execute(uow, req).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(crate::domain::resources::update::Error::NotFound) => Err(ApiError::NotFound),
        Err(crate::domain::resources::update::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::update::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e.to_string()))
        }
    }
}
