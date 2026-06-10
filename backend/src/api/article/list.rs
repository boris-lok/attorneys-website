use crate::api::api_error::ApiError;
use crate::api::extractors::query::QueryExtractor;
use crate::domain::articles::entity::SimpleArticleEntity;
use crate::domain::entity::{Page, Pagination};
use crate::domain::resources::entity::{Language, ResourceType};
use crate::domain::resources::list;
use axum::extract::Query;
use axum::http::HeaderMap;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryPagination {
    page: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Deserialize)]
pub struct CategoryQuery {
    category_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListArticlesResponse {
    articles: Vec<SimpleArticleEntity>,
    total: usize,
}

pub async fn list_articles(
    QueryExtractor(query): QueryExtractor,
    headers: HeaderMap,
    pagination: Query<QueryPagination>,
    category_query: Query<CategoryQuery>,
) -> Result<Json<ListArticlesResponse>, ApiError> {
    let id = category_query.category_id.clone();

    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = list::Request {
        filter_str: id.map(|s| format!(" and content.data->>'category_id' = '{}'", s)),
        kind: ResourceType::Article,
        language: Language::try_from(lang.to_string()).map_err(|_| ApiError::BadRequest)?,
        default_language: Language::ZH,
        pagination: Pagination::Page(Page {
            page: pagination.page.unwrap_or(0),
            size: pagination.page_size.unwrap_or(10),
        }),
    };

    match list::execute(&query, req).await {
        Ok((articles, total)) => Ok(Json(ListArticlesResponse { articles, total })),
        Err(list::Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
    }
}
