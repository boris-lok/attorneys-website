use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::entities::{HomeData, Resource};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use ulid::Ulid;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateHomeRequest {
    data: String,
    language: String,
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
    let home_id = Ulid::new().to_string();

    let req = crate::domain::resources::create::Request {
        id: home_id.clone(),
        data: Resource::Home(HomeData::new(req.data)),
        language: req.language,
    };

    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    match crate::domain::resources::create::execute(uow, req).await {
        Ok(id) => Ok(Json(CreateHomeResponse { id: id.to_string() })),
        Err(crate::domain::resources::create::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::create::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
