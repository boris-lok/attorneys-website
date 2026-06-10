use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::entity::Pagination;
use crate::domain::resources::entity::{Language, ResourceType};
use crate::domain::resources::list;
use crate::domain::services::entity::ServiceEntity;
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ListServicesResponse {
    services: Vec<ServiceEntity>,
}

pub async fn list_services(
    QueryExtractor(query): QueryExtractor,
    headers: HeaderMap,
) -> Result<Json<ListServicesResponse>, ApiError> {
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = list::Request {
        filter_str: None,
        kind: ResourceType::Service,
        language: Language::try_from(lang.to_string()).map_err(|_| ApiError::BadRequest)?,
        default_language: Language::ZH,
        pagination: Pagination::All,
    };

    match list::execute(&query, req).await {
        Ok((services, _)) => Ok(Json(ListServicesResponse { services })),
        Err(list::Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
