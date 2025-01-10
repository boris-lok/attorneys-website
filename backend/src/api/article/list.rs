use crate::api::api_error::ApiError;
use crate::domain::entities::{ArticleEntity, Language, Page, Pagination, ResourceType};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::Json;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct QueryPagination {
    page: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ListArticlesResponse {
    articles: Vec<ArticleEntity>,
    total: usize,
}

pub async fn list_articles(
    State(state): State<AppState>,
    headers: HeaderMap,
    pagination: Query<QueryPagination>,
) -> Result<Json<ListArticlesResponse>, ApiError> {
    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = crate::domain::resources::list::Request {
        resource_type: ResourceType::Article,
        language: lang.to_string(),
        default_language: Language::ZH,
        pagination: Pagination::Page(Page {
            page: pagination.page.unwrap_or(0),
            size: pagination.page_size.unwrap_or(10),
        }),
    };

    match crate::domain::resources::list::execute(uow, req).await {
        Ok((articles, total)) => Ok(Json(ListArticlesResponse { articles, total })),
        Err(crate::domain::resources::list::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::list::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e.to_string()))
        }
    }
}
