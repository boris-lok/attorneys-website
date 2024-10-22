use crate::api::api_error::ApiError;
use crate::domain::entities::{Language, Pagination, ResourceType, SimpleMemberEntity};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub(crate) struct ListMembersResponse {
    members: Vec<SimpleMemberEntity>,
}
pub async fn list_members(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ListMembersResponse>, ApiError> {
    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = crate::domain::resources::list::Request {
        resource_type: ResourceType::Member,
        language: lang.to_string(),
        default_language: Language::ZH,
        pagination: Pagination::All,
    };

    match crate::domain::resources::list::execute(uow, req).await {
        Ok((members, _)) => Ok(Json(ListMembersResponse { members })),
        Err(crate::domain::resources::list::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::list::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e.to_string()))
        }
    }
}
