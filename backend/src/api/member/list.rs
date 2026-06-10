use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::entity::Pagination;
use crate::domain::member::entity::SimpleMemberEntity;
use crate::domain::resources::entity::{Language, ResourceType};
use crate::domain::resources::list;
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ListMembersResponse {
    members: Vec<SimpleMemberEntity>,
}
pub async fn list_members(
    QueryExtractor(query): QueryExtractor,
    headers: HeaderMap,
) -> Result<Json<ListMembersResponse>, ApiError> {
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = list::Request {
        filter_str: None,
        kind: ResourceType::Member,
        language: Language::try_from(lang.to_string()).map_err(|_| ApiError::BadRequest)?,
        default_language: Language::ZH,
        pagination: Pagination::All,
    };

    match list::execute(&query, req).await {
        Ok((members, _)) => Ok(Json(ListMembersResponse { members })),
        Err(list::Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
