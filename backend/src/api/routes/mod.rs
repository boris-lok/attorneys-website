mod admin;
mod middleware;
mod public;

use crate::startup::AppState;
use crate::utils::image::ImageUtil;
use axum::routing::get;
use axum::{Extension, Router};
use std::sync::Arc;

pub fn build_router(state: AppState, redis_client: redis::Client, image_util: ImageUtil) -> Router {
    use crate::api::health::health_check;

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/{version}/admin", admin::router())
        .nest("/api/{version}/", public::router())
        .layer(Extension(Arc::new(image_util)))
        .layer(Extension(Arc::new(redis_client)))
        .layer(middleware::cors_layer())
        .with_state(state)
}
