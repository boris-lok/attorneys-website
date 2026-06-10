use crate::api::resources::delete::delete_resource;
use crate::api::service::create::create_service;
use crate::api::service::update::update_service;
use crate::startup::AppState;
use axum::routing::{delete, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/services", post(create_service).put(update_service))
        .route("/services/{id}", delete(delete_resource))
}
