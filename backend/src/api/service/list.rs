use crate::api::api_error::ApiError;
use crate::domain::member::entities::Language;
use crate::domain::service::entities::Service;
use crate::startup::AppState;
use crate::uow::service::SqlxServiceUnitOfWork;
use axum::extract::{Path, State};
use axum::Json;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub(crate) struct ListServicesResponse {
    services: Vec<Service>,
}

pub async fn list_services(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<ListServicesResponse>, ApiError> {
    let uow = SqlxServiceUnitOfWork::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let lang = params.get("lang").ok_or(ApiError::BadRequest)?;
    let req = crate::domain::service::list::Request {
        language: lang.to_string(),
        default_language: Language::ZH,
    };

    match crate::domain::service::list::execute(uow, req).await {
        Ok(services) => Ok(Json(ListServicesResponse { services })),
        Err(crate::domain::service::list::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::service::list::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e.to_string()))
        }
    }
}
