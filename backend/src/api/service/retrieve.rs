use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::resources::entity::{Language, ResourceID, ResourceType};
use crate::domain::resources::retrieve;
use crate::domain::services::entity::ServiceEntity;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub(crate) struct RetrieveServiceResponse {
    service: ServiceEntity,
}

pub async fn retrieve_service(
    QueryExtractor(query): QueryExtractor,
    Path(params): Path<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Json<RetrieveServiceResponse>, ApiError> {
    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = retrieve::Request {
        id: ResourceID::try_from(id.clone()).map_err(|_| ApiError::BadRequest)?,
        resource_type: ResourceType::Service,
        language: Language::try_from(lang.to_string()).map_err(|_| ApiError::BadRequest)?,
        default_language: Language::ZH,
    };

    match retrieve::execute::<ServiceEntity>(&query, req).await {
        Ok(service) => Ok(Json(RetrieveServiceResponse { service })),
        Err(retrieve::Error::NotFound) => Err(ApiError::NotFound),
        Err(retrieve::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
