mod admin;
mod middleware;
mod public;

use crate::api::routes::admin::CasesApi;
use crate::startup::AppState;
use crate::utils::image::ImageUtil;
use axum::routing::get;
use axum::{Extension, Router};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
struct Api;

pub fn build_router(state: AppState, image_util: ImageUtil) -> Router {
    use crate::api::health::health_check;
    let mut openapi = Api::openapi();
    openapi.merge(CasesApi::openapi());

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/{version}/admin", admin::router())
        .nest("/api/{version}/", public::router())
        .merge(SwaggerUi::new("/swagger-ui").url("/swagger-ui.json", openapi))
        .layer(Extension(Arc::new(image_util)))
        .layer(middleware::cors_layer())
        .with_state(state)
}
