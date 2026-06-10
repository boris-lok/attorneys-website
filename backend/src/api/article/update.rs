use crate::api::api_error::ApiError;
use crate::api::resources::update::execute_update;
use crate::domain::articles::entity::ArticleData;
use crate::domain::entity::Claims;
use crate::domain::resources::entity::Resource;
use crate::startup::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    id: String,
    category_id: Option<String>,
    title: String,
    content: String,
    language: String,
    seq: i32,
}

pub async fn update_article(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateArticleRequest>, ApiError>,
) -> Result<StatusCode, ApiError> {
    let resource = Resource::Article(ArticleData {
        category_id: req.category_id,
        title: req.title,
        content: req.content,
    });

    execute_update(&state, req.id, req.seq, req.language, resource).await
}
