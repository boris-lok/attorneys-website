use crate::api::contact::list::list_contact;
use crate::api::contact::retrieve::retrieve_contact;
use crate::startup::AppState;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/contact/{id}", get(retrieve_contact))
        .route("/contact", get(list_contact))
}
