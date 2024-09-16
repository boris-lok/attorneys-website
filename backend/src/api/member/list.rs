use crate::api::api_error::ApiError;
use crate::domain::member::entities::{Language, SimpleMember};
use crate::domain::member::list::Error;
use crate::startup::AppState;
use crate::uow::member::SqlxMemberUnitOfWork;
use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub(crate) struct ListMembersResponse {
    members: Vec<SimpleMember>,
}
pub async fn list_members(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<ListMembersResponse>, ApiError> {
    let uow = SqlxMemberUnitOfWork::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let lang = params.get("lang").ok_or(ApiError::BadRequest)?;
    let req = crate::domain::member::list::Request {
        language: lang.to_string(),
        default_language: Language::ZH,
    };

    match crate::domain::member::list::execute(uow, req).await {
        Ok(members) => Ok(Json(ListMembersResponse { members })),
        Err(Error::BadRequest) => Err(ApiError::BadRequest),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
