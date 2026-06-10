use crate::api::categories::list::list_categories;
use crate::api::categories::retrieve::retrieve_category;
use crate::startup::AppState;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list_categories))
        .route("/categories/{id}", get(retrieve_category))
}
