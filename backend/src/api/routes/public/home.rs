use crate::api::home::list::list_home;
use crate::api::home::retrieve::retrieve_home;
use crate::startup::AppState;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/home/{id}", get(retrieve_home))
        .route("/home", get(list_home))
}
