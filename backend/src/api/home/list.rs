use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::entity::Pagination;
use crate::domain::resources::entity::{HomeEntity, Language, ResourceType};
use crate::domain::resources::list;
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ListHomeResponse {
    home: Vec<HomeEntity>,
}

pub async fn list_home(
    QueryExtractor(query): QueryExtractor,
    headers: HeaderMap,
) -> Result<Json<ListHomeResponse>, ApiError> {
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = list::Request {
        filter_str: None,
        kind: ResourceType::Home,
        language: Language::try_from(lang.to_string()).map_err(|_| ApiError::BadRequest)?,
        default_language: Language::ZH,
        pagination: Pagination::Single,
    };

    match list::execute(&query, req).await {
        Ok((home, _)) => Ok(Json(ListHomeResponse { home })),
        Err(list::Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
