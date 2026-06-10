use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::member::entity::MemberData;
use crate::domain::resources::create;
use crate::domain::resources::entity::Language;
use crate::domain::resources::entity::Resource;
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct CreateMemberRequest {
    name: String,
    description: String,
    language: String,
    seq: i32,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreateMemberResponse {
    id: String,
}

pub async fn create_member(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateMemberRequest>, ApiError>,
) -> Result<Json<CreateMemberResponse>, ApiError> {
    let resource = Resource::Member(MemberData {
        name: req.name,
        description: req.description,
    });

    let (kind, data) = resource
        .into_typed_content()
        .map_err(|_| ApiError::BadRequest)?;

    let req = create::Request {
        kind,
        data,
        seq: req.seq,
        language: Language::try_from(req.language).map_err(|_| ApiError::BadRequest)?,
    };

    let service = state.resource_uow();

    match create::execute(&service, req).await {
        Ok(id) => Ok(Json(CreateMemberResponse { id: id.to_string() })),
        Err(create::Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
