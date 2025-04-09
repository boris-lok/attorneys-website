use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::api::update_resource_handler;
use crate::domain::entities::{CategoryData, Resource};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use tokio::sync::Mutex;

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
    let req = crate::domain::resources::update::Request {
        id: req.id,
        data: Resource::Category(CategoryData::new(req.icon, req.name)),
        language: req.language,
        seq: req.seq,
    };

    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    update_resource_handler(uow, req).await
}
