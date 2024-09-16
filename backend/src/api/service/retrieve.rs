use crate::api::api_error::ApiError;
use crate::domain::member::entities::Language;
use crate::domain::service::entities::Service;
use crate::startup::AppState;
use crate::uow::service::SqlxServiceUnitOfWork;
use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Serialize)]
pub(crate) struct RetrieveServiceResponse {
    service: Service,
}

pub async fn retrieve_service(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<RetrieveServiceResponse>, ApiError> {
    let uow = SqlxServiceUnitOfWork::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);
    let service_id = params.get("id").ok_or(ApiError::BadRequest)?;
    let lang = params.get("lang").ok_or(ApiError::BadRequest)?;

    let req = crate::domain::service::retrieve::Request {
        service_id: service_id.to_string(),
        language: lang.to_string(),
        default_language: Language::ZH,
    };

    match crate::domain::service::retrieve::execute(uow, req).await {
        Ok(service) => match service {
            None => Err(ApiError::NotFound),
            Some(s) => Ok(Json(RetrieveServiceResponse { service: s })),
        },
        Err(crate::domain::service::retrieve::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::service::retrieve::Error::ServiceNotFound) => Err(ApiError::NotFound),
        Err(crate::domain::service::retrieve::Error::Unknown) => {
            Err(ApiError::InternalServerError("Unknown error".to_string()))
        }
    }
}
