use crate::api::api_error::ApiError;
use crate::domain::create_member::{execute, Request, Response};
use crate::repositories::member_repository::MemberRepository;
use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ulid::Ulid;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateMemberRequest {
    name: String,
    language: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreateMemberResponse {
    member_id: String,
}

pub async fn create_member(
    Extension(member_repo): Extension<Arc<dyn MemberRepository + Sync + Send>>,
    WithRejection(Json(req), _): WithRejection<Json<CreateMemberRequest>, ApiError>,
) -> Result<Json<CreateMemberResponse>, ApiError> {
    let member_id = Ulid::new().to_string();

    let request = Request {
        member_id,
        name: req.name,
        language: req.language,
    };

    match execute(member_repo, request) {
        Response::BadRequest => Err(ApiError::BadRequest),
        Response::Conflict => Err(ApiError::MemberAlreadyExists),
        Response::Error => Err(ApiError::InternalServerError),
        Response::Ok(id) => Ok(Json(CreateMemberResponse { member_id: id })),
    }
}
