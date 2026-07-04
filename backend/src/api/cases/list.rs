use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::cases::entity::Case;
use crate::domain::cases::list::execute;
use crate::domain::entity::Claims;
use crate::domain::users::entity::UserID;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ListCasesResponse {
    cases: Vec<Case>,
}

#[utoipa::path(
    get,
    path = "/cases",
    responses(
        (status = 200, description = "List of cases", body = ListCasesResponse),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal Server Error", body = ApiError)
    )
)]
pub async fn list_cases(
    c: Claims,
    QueryExtractor(query): QueryExtractor,
) -> Result<Json<ListCasesResponse>, ApiError> {
    let user_id = UserID::try_from(c.sub.clone()).map_err(|_| ApiError::BadRequest)?;

    let res = execute(&query, &user_id).await?;

    Ok(Json(ListCasesResponse { cases: res }))
}
