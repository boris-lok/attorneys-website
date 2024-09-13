use crate::api::api_error::ApiError;
use crate::domain::entities::{Language, Member};
use crate::domain::get_member::Error;
use crate::startup::AppState;
use crate::uow::member::SqlxMemberUnitOfWork;
use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub(crate) struct GetMemberResponse {
    member: Member,
}

pub async fn get_member(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<GetMemberResponse>, ApiError> {
    let uow = SqlxMemberUnitOfWork::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);
    let member_id = params.get("id").ok_or(ApiError::BadRequest)?;
    let lang = params.get("lang").ok_or(ApiError::BadRequest)?;

    let req = crate::domain::get_member::Request {
        member_id: member_id.to_string(),
        language: lang.to_string(),
        default_language: Language::ZH,
    };

    match crate::domain::get_member::execute(uow, req).await {
        Ok(member) => match member {
            None => Err(ApiError::NotFound),
            Some(member) => Ok(Json(GetMemberResponse { member })),
        },
        Err(Error::MemberNotFound) => Err(ApiError::NotFound),
        Err(Error::BadRequest) => Err(ApiError::BadRequest),
        Err(Error::Unknown) => Err(ApiError::InternalServerError("Unknown error".to_string())),
    }
}
