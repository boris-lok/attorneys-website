use crate::api::contact::create::create_contact;
use crate::api::contact::update::update_contact;
use crate::startup::AppState;
use axum::routing::post;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().route("/contact", post(create_contact).put(update_contact))
}
