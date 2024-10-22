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
    page: u32,
    page_size: u32,
}

impl Default for QueryPagination {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 10,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListArticlesResponse {
    articles: Vec<ArticleEntity>,
    total: usize,
}

pub async fn list_articles(
    State(state): State<AppState>,
    pagination: Option<Query<QueryPagination>>,
    headers: HeaderMap,
) -> Result<Json<ListArticlesResponse>, ApiError> {
    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let pagination = pagination.unwrap_or_default();

    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = crate::domain::resources::list::Request {
        resource_type: ResourceType::Article,
        language: lang.to_string(),
        default_language: Language::ZH,
        pagination: Pagination::Page(Page {
            page: pagination.page,
            size: pagination.page_size,
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
