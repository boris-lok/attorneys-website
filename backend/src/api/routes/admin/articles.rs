use crate::api::article::create::create_article;
use crate::api::article::update::update_article;
use crate::api::resources::delete::delete_resource;
use crate::startup::AppState;
use axum::routing::{delete, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/articles", post(create_article).put(update_article))
        .route("/articles/{id}", delete(delete_resource))
}
