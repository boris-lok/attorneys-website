use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::{CategoryData, Resource};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use ulid::Ulid;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateCategoryRequest {
    icon: Option<String>,
    name: String,
    language: String,
    seq: i32,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreateCategoryResponse {
    id: String,
}

pub async fn create_category(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateCategoryRequest>, ApiError>,
) -> Result<Json<CreateCategoryResponse>, ApiError> {
    let category_id = Ulid::new().to_string();

    let req = crate::domain::resources::create::Request {
        id: category_id.clone(),
        data: Resource::Category(CategoryData::new(req.icon, req.name)),
        language: req.language,
        seq: req.seq,
    };

    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    match crate::domain::resources::create::execute(uow, req).await {
        Ok(id) => Ok(Json(CreateCategoryResponse { id: id.to_string() })),
        Err(crate::domain::resources::create::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::create::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
