use crate::api::api_error::ApiError;
use crate::startup::AppState;
use crate::uow::home::SqlxHomeUnitOfWork;
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
    home_id: String,
}

pub async fn create_home(
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateHomeRequest>, ApiError>,
) -> Result<Json<CreateHomeResponse>, ApiError> {
    let home_id = Ulid::new().to_string();

    let req = crate::domain::home::create::Request {
        home_id: home_id.clone(),
        data: req.data,
        language: req.language,
    };

    let uow = SqlxHomeUnitOfWork::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    match crate::domain::home::create::execute(uow, req).await {
        Ok(id) => Ok(Json(CreateHomeResponse { home_id: id })),
        Err(crate::domain::home::create::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::home::create::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
