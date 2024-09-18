use crate::api::api_error::ApiError;
use crate::domain::home::entities::Home;
use crate::domain::member::entities::Language;
use crate::startup::AppState;
use crate::uow::home::SqlxHomeUnitOfWork;
use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub(crate) struct ListServicesResponse {
    services: Vec<Home>,
}

pub async fn list_home(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<ListServicesResponse>, ApiError> {
    let uow = SqlxHomeUnitOfWork::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let lang = params.get("lang").ok_or(ApiError::BadRequest)?;
    let req = crate::domain::home::list::Request {
        language: lang.to_string(),
        default_language: Language::ZH,
    };

    match crate::domain::home::list::execute(uow, req).await {
        Ok(services) => Ok(Json(ListServicesResponse { services })),
        Err(crate::domain::home::list::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::home::list::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e.to_string()))
        }
    }
}
