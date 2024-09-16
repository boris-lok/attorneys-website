use crate::api::{
    create_home, create_member, create_service, delete_member, health_check, list_members,
    list_services, retrieve_home, retrieve_member, retrieve_service, update_home, update_member,
    update_service, upload_member_avatar,
};
use crate::configuration::{DatabaseSettings, Settings};
use crate::utils::image::ImageUtil;
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn run(config: Settings, listener: TcpListener) -> Result<(), std::io::Error> {
    let state = AppState {
        pool: get_database_connection(&config.database).await,
    };
    let image_util = ImageUtil {};
    let member_routes = Router::new()
        .route("/members", post(create_member))
        .route("/members/:id/avatar", post(upload_member_avatar))
        .route("/members/:id", delete(delete_member))
        .route("/members", put(update_member))
        .route("/members/:lang", get(list_members))
        .route("/members/:lang/:id", get(retrieve_member));

    let service_routes = Router::new()
        .route("/services", post(create_service))
        .route("/services", put(update_service))
        .route("/services/:lang/:id", get(retrieve_service))
        .route("/services/:lang", get(list_services));

    let home_routes = Router::new()
        .route("/home", post(create_home))
        .route("/home", put(update_home))
        .route("/home/:lang/:id", get(retrieve_home));

    let admin_routes = Router::new()
        .merge(member_routes)
        .merge(service_routes)
        .merge(home_routes);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/:version/admin", admin_routes)
        .layer(CorsLayer::permissive())
        .layer(Extension(Arc::new(image_util)))
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await
}

pub async fn get_database_connection(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}
