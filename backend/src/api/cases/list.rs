use crate::api::api_error::ApiError;
use crate::api::auth::Claims;
use crate::domain::cases::entity::Case;
use crate::domain::cases::list::{execute, Error};
use crate::domain::users::entity::UserID;
use crate::infrastructure::db::case_repo::PostgresCaseRepo;
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListCasesResponse {
    cases: Vec<Case>,
}

pub async fn list_cases(
    c: Claims,
    State(state): State<AppState>,
) -> Result<Json<ListCasesResponse>, ApiError> {
    let mut repo = PostgresCaseRepo::new(&state.pool)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let user_id = UserID::try_from(c.sub.clone()).map_err(|_| ApiError::BadRequest)?;

    let res = execute(&mut repo, &user_id).await;

    match res {
        Ok(cases) => Ok(Json(ListCasesResponse { cases })),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
