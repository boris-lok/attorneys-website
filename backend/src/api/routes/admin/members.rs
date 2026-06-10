use crate::api::member::create::create_member;
use crate::api::member::update::update_member;
use crate::api::member::upload_avatar::upload_member_avatar;
use crate::api::resources::delete::delete_resource;
use crate::api::routes::middleware::upload_body_limit;
use crate::startup::AppState;
use axum::routing::{delete, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/members", post(create_member).put(update_member))
        .route("/members/{id}", delete(delete_resource))
        .route("/members/{id}/avatar", post(upload_member_avatar))
        .layer(upload_body_limit())
}
