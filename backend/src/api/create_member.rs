use crate::api::api_error::ApiError;
use crate::domain::create_member;
use crate::domain::create_member::{execute, Request};
use crate::repositories::member_repository::IMemberRepository;
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
    Extension(member_repo): Extension<Arc<dyn IMemberRepository + Sync + Send>>,
    WithRejection(Json(req), _): WithRejection<Json<CreateMemberRequest>, ApiError>,
) -> Result<Json<CreateMemberResponse>, ApiError> {
    let member_id = Ulid::new().to_string();

    Ok(Json(CreateMemberResponse { member_id }))

    // let request = Request {
    //     member_id,
    //     language: req.language,
    // };
    //
    // match execute(member_repo, request) {
    //     Ok(id) => Ok(Json(CreateMemberResponse { member_id: id })),
    //     Err(create_member::Error::BadRequest) => Err(ApiError::BadRequest),
    //     Err(create_member::Error::Conflict) => Err(ApiError::MemberAlreadyExists),
    //     Err(create_member::Error::Unknown) => Err(ApiError::InternalServerError),
    // }
}
