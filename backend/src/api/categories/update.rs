use crate::api::api_error::ApiError;
use crate::api::resources::update::execute_update;
use crate::domain::articles::entity::CategoryData;
use crate::domain::entity::Claims;
use crate::domain::resources::entity::Resource;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateCategoryRequest {
    id: String,
    icon: Option<String>,
    name: String,
    language: String,
    seq: i32,
}

pub async fn update_category(
    _: Claims,
    state: State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateCategoryRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let resource = Resource::Category(CategoryData {
        icon: req.icon,
        name: req.name,
    });

    execute_update(&state, req.id, req.seq, req.language, resource).await
}
