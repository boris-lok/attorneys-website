use crate::api::api_error::ApiError;
use crate::api::resources::update::execute_update;
use crate::domain::entity::Claims;
use crate::domain::member::entity::MemberData;
use crate::domain::resources::entity::Resource;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateMemberRequest {
    id: String,
    name: String,
    description: String,
    language: String,
    seq: i32,
}

pub async fn update_member(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateMemberRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let resource = Resource::Member(MemberData {
        name: req.name,
        description: req.description,
    });

    execute_update(&state, req.id, req.seq, req.language, resource).await
}
