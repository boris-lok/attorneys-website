use crate::api::api_error::ApiError;
use crate::domain::entities::ResourceType;
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub async fn delete_service(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<StatusCode, ApiError> {
    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let id = params.get("id").ok_or(ApiError::BadRequest)?;
    let req = crate::domain::resources::delete::Request {
        id: id.to_string(),
        resource_type: ResourceType::Service,
    };

    match crate::domain::resources::delete::execute(uow, req).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(crate::domain::resources::delete::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::delete::Error::NotFound) => Err(ApiError::NotFound),
        Err(crate::domain::resources::delete::Error::Unknown(reason)) => {
            Err(ApiError::InternalServerError(reason))
        }
    }
}