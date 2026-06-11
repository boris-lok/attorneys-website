use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::cases::entity::Case;
use crate::domain::cases::list::{execute, Error};
use crate::domain::entity::Claims;
use crate::domain::users::entity::UserID;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListCasesResponse {
    cases: Vec<Case>,
}

pub async fn list_cases(
    c: Claims,
    QueryExtractor(query): QueryExtractor,
) -> Result<Json<ListCasesResponse>, ApiError> {
    let user_id = UserID::try_from(c.sub.clone()).map_err(|_| ApiError::BadRequest)?;

    let res = execute(&query, &user_id).await;

    match res {
        Ok(cases) => Ok(Json(ListCasesResponse { cases })),
        Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
