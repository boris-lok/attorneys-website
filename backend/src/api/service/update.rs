use crate::api::api_error::ApiError;
use crate::startup::AppState;
use crate::uow::service::SqlxServiceUnitOfWork;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateServiceRequest {
    service_id: String,
    data: String,
    language: String,
}

pub async fn update_service(
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateServiceRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let req = crate::domain::service::update::Request {
        service_id: req.service_id,
        data: req.data,
        language: req.language,
    };

    let uow = SqlxServiceUnitOfWork::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    match crate::domain::service::update::execute(uow, req).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(crate::domain::service::update::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::service::update::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e.to_string()))
        }
    }
}
