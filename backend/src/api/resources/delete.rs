use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::resources::delete;
use crate::domain::resources::entity::ResourceID;
use crate::startup::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;

pub async fn delete_resource(
    _: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let resource_id = ResourceID::try_from(id).map_err(|_| ApiError::BadRequest)?;

    let service = state.resource_uow();

    let res = delete::execute(&service, &resource_id).await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(delete::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
