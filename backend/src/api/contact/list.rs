use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::entity::Pagination;
use crate::domain::resources::entity::{ContactEntity, Language, ResourceType};
use crate::domain::resources::list;
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListContactResponse {
    contact: Vec<ContactEntity>,
}

pub async fn list_contact(
    QueryExtractor(query): QueryExtractor,
    headers: HeaderMap,
) -> Result<Json<ListContactResponse>, ApiError> {
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = list::Request {
        filter_str: None,
        kind: ResourceType::Contact,
        language: Language::try_from(lang.to_string()).map_err(|_| ApiError::BadRequest)?,
        default_language: Language::ZH,
        pagination: Pagination::Single,
    };

    match list::execute(&query, req).await {
        Ok((contact, _)) => Ok(Json(ListContactResponse { contact })),
        Err(list::Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
