use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::resources::create;
use crate::domain::resources::entity::{HomeData, Language, Resource};
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct CreateHomeRequest {
    data: String,
    language: String,
    seq: i32,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreateHomeResponse {
    id: String,
}

pub async fn create_home(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateHomeRequest>, ApiError>,
) -> Result<Json<CreateHomeResponse>, ApiError> {
    let resource = Resource::Home(HomeData { data: req.data });

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
        Ok(id) => Ok(Json(CreateHomeResponse { id: id.to_string() })),
        Err(create::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
