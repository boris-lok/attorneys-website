use crate::api::article::list::list_articles;
use crate::api::article::retrieve::retrieve_article;
use crate::api::article::view::view_article;
use crate::startup::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/articles/{id}/view", post(view_article))
        .route("/articles/{id}", get(retrieve_article))
        .route("/articles", get(list_articles))
}
