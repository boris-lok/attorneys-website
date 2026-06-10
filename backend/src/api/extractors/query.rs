use crate::api::api_error::ApiError;
use crate::infrastructure::db::uow::PostgresQuery;
use crate::startup::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

pub struct QueryExtractor(pub PostgresQuery);

impl FromRequestParts<AppState> for QueryExtractor {
    type Rejection = ApiError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(PostgresQuery::new(state.pool.clone())))
    }
}
