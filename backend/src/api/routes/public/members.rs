use crate::api::member::list::list_members;
use crate::api::member::retrieve::retrieve_member;
use crate::startup::AppState;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/members/{id}", get(retrieve_member))
        .route("/members", get(list_members))
}
