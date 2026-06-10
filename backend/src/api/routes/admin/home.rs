use crate::api::home::create::create_home;
use crate::api::home::update::update_home;
use crate::startup::AppState;
use axum::routing::post;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().route("/home", post(create_home).put(update_home))
}
