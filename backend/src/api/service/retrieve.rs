use crate::api::api_error::ApiError;
use crate::domain::entities::{Language, ResourceType, ServiceEntity};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Serialize)]
pub(crate) struct RetrieveServiceResponse {
    service: ServiceEntity,
}

pub async fn retrieve_service(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Json<RetrieveServiceResponse>, ApiError> {
    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let service_id = params.get("id").ok_or(ApiError::BadRequest)?;
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = crate::domain::resources::retrieve::Request {
        id: service_id.to_string(),
        resource_type: ResourceType::Service,
        language: lang.to_string(),
        default_language: Language::ZH,
    };

    match crate::domain::resources::retrieve::execute::<InDatabase, ServiceEntity>(uow, req).await {
        Ok(service) => Ok(Json(RetrieveServiceResponse { service })),
        Err(crate::domain::resources::retrieve::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::retrieve::Error::NotFound) => Err(ApiError::NotFound),
        Err(crate::domain::resources::retrieve::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
