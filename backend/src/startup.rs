use crate::api::health_check;
use axum::routing::get;
use axum::{Router, ServiceExt};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let app = Router::new().route("/health", get(health_check));

    axum::serve(listener, app.into_make_service()).await
}
