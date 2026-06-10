use axum::extract::DefaultBodyLimit;
use axum::http::header::{ACCEPT_LANGUAGE, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub fn cors_layer() -> CorsLayer {
    let allowed_origins = ["http://localhost:5173", "https://chenwanglaw.com"];

    CorsLayer::new()
        .allow_origin(
            allowed_origins
                .into_iter()
                .map(|o| o.parse::<HeaderValue>().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, ACCEPT_LANGUAGE])
}

pub fn upload_body_limit() -> DefaultBodyLimit {
    DefaultBodyLimit::max(5 * 1024 * 1024)
}
