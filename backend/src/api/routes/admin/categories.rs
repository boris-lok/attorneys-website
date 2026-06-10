use crate::api::categories::create::create_category;
use crate::api::categories::update::update_category;
use crate::api::resources::delete::delete_resource;
use crate::startup::AppState;
use axum::routing::{delete, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/categories", post(create_category).put(update_category))
        .route("/categories/{id}", delete(delete_resource))
}
