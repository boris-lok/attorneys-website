use crate::api::api_error::ApiError;
use crate::domain::articles::entity::ArticleData;
use crate::domain::entity::Claims;
use crate::domain::resources::create;
use crate::domain::resources::entity::{Language, Resource};
use crate::startup::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct CreateArticleRequest {
    category_id: Option<String>,
    title: String,
    content: String,
    language: String,
    seq: i32,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreateArticleResponse {
    id: String,
}

pub async fn create_article(
    _: Claims,
    State(state): State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<CreateArticleRequest>, ApiError>,
) -> Result<Json<CreateArticleResponse>, ApiError> {
    let resource = Resource::Article(ArticleData {
        category_id: req.category_id,
        title: req.title,
        content: req.content,
    });

    let (kind, data) = resource
        .into_typed_content()
        .map_err(|_| ApiError::BadRequest)?;

    let req = create::Request {
        kind,
        data,
        seq: req.seq,
        language: Language::try_from(req.language).map_err(|_| ApiError::BadRequest)?,
    };

    let uow = state.resource_uow();

    match create::execute(&uow, req).await {
        Ok(id) => Ok(Json(CreateArticleResponse { id: id.to_string() })),
        Err(create::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
    }
}
