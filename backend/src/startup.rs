use crate::api::{create_member, health_check};
use crate::repositories::member_repository::{InMemoryMemberRepository, MemberRepository};
use axum::routing::{get, post};
use axum::{Extension, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let member_repo =
        Arc::new(InMemoryMemberRepository::new()) as Arc<dyn MemberRepository + Send + Sync>;

    let member_routes = Router::new().route("/members", post(create_member));

    let admin_routes = Router::new().merge(member_routes);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/:version/admin", admin_routes)
        .layer(Extension(member_repo))
        .layer(CorsLayer::permissive());

    axum::serve(listener, app.into_make_service()).await
}
