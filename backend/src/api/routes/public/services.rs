use crate::api::service::list::list_services;
use crate::api::service::retrieve::retrieve_service;
use crate::startup::AppState;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/services/{id}", get(retrieve_service))
        .route("/services", get(list_services))
}
