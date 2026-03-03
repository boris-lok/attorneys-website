use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::cases::list::{execute, Error};
use crate::repositories::{Case, SQLxCaseRepository};
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct ListCasesResponse {
    cases: Vec<Case>,
}

pub async fn list_cases(
    _: Claims,
    State(state): State<AppState>,
) -> Result<Json<ListCasesResponse>, ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let repo = SQLxCaseRepository::new(Arc::new(Mutex::new(&mut *conn)));

    let res = execute(Arc::new(Mutex::new(repo))).await;

    match res {
        Ok(cases) => Ok(Json(ListCasesResponse { cases })),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
