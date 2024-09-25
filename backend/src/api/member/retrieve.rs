use crate::api::api_error::ApiError;
use crate::domain::member::entities::{Language, Member};
use crate::domain::member::retrieve::Error;
use crate::startup::AppState;
use crate::uow::member::SqlxMemberUnitOfWork;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub(crate) struct RetrieveMemberResponse {
    member: Member,
}

pub async fn retrieve_member(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Json<RetrieveMemberResponse>, ApiError> {
    let uow = SqlxMemberUnitOfWork::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let member_id = params.get("id").ok_or(ApiError::BadRequest)?;
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = crate::domain::member::retrieve::Request {
        member_id: member_id.to_string(),
        language: lang.to_string(),
        default_language: Language::ZH,
    };

    match crate::domain::member::retrieve::execute(uow, req).await {
        Ok(member) => match member {
            None => Err(ApiError::NotFound),
            Some(member) => Ok(Json(RetrieveMemberResponse { member })),
        },
        Err(Error::MemberNotFound) => Err(ApiError::NotFound),
        Err(Error::BadRequest) => Err(ApiError::BadRequest),
        Err(Error::Unknown) => Err(ApiError::InternalServerError("Unknown error".to_string())),
    }
}
