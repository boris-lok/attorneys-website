use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::resources::entity::{ContactEntity, Language, ResourceID, ResourceType};
use crate::domain::resources::retrieve;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct RetrieveContactResponse {
    contact: ContactEntity,
}

pub async fn retrieve_contact(
    QueryExtractor(query): QueryExtractor,
    Path(params): Path<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Json<RetrieveContactResponse>, ApiError> {
    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = retrieve::Request {
        id: ResourceID::try_from(id.to_string()).map_err(|_| ApiError::BadRequest)?,
        resource_type: ResourceType::Contact,
        language: Language::try_from(lang.to_string()).map_err(|_| ApiError::BadRequest)?,
        default_language: Language::ZH,
    };

    match retrieve::execute::<ContactEntity>(&query, req).await {
        Ok(res) => Ok(Json(RetrieveContactResponse { contact: res })),
        Err(retrieve::Error::NotFound) => Err(ApiError::NotFound),
        Err(retrieve::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
