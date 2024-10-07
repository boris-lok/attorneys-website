use crate::api::api_error::ApiError;
use crate::domain::entities::{ArticleEntity, Language, ResourceType};
use crate::startup::AppState;
use crate::uow::InDatabase;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Serialize)]
pub(crate) struct RetrieveArticleResponse {
    article: ArticleEntity,
}

pub async fn retrieve_article(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Json<RetrieveArticleResponse>, ApiError> {
    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);

    let home_id = params.get("id").ok_or(ApiError::BadRequest)?;
    let lang = headers
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("zh");

    let req = crate::domain::resources::retrieve::Request {
        id: home_id.to_string(),
        resource_type: ResourceType::Article,
        language: lang.to_string(),
        default_language: Language::ZH,
    };

    match crate::domain::resources::retrieve::execute::<InDatabase, ArticleEntity>(uow, req).await {
        Ok(res) => Ok(Json(RetrieveArticleResponse { article: res })),
        Err(crate::domain::resources::retrieve::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::retrieve::Error::NotFound) => Err(ApiError::NotFound),
        Err(crate::domain::resources::retrieve::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
