use crate::api::{
    create_home, create_member, create_service, health_check, retrieve_home, retrieve_member,
    retrieve_service, upload_member_avatar,
};
use crate::configuration::{DatabaseSettings, Settings};
use crate::utils::image::ImageUtil;
use axum::routing::{get, post};
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

    // Config the routes
    let admin_member_routes = Router::new()
        .route("/members", post(create_member))
        //     .route("/members/:id", delete(delete_member))
        .route("/members/:id/avatar", post(upload_member_avatar));
    let member_routes = Router::new().route("/members/:id", get(retrieve_member));
    //     .route("/members", get(list_members));
    //
    let admin_service_routes = Router::new().route("/services", post(create_service));
    let service_routes = Router::new().route("/services/:id", get(retrieve_service));
    //     .route("/services", get(list_services));
    //
    let admin_home_routes = Router::new().route("/home", post(create_home));
    let home_routes = Router::new().route("/home/:id", get(retrieve_home));
    //     .route("/home", get(list_home));
    //
    let admin_routes = Router::new()
        .merge(admin_member_routes)
        .merge(admin_home_routes)
        .merge(admin_service_routes);

    let routes = Router::new()
        .merge(member_routes)
        .merge(service_routes)
        .merge(home_routes);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/:version/admin", admin_routes)
        .nest("/api/:version/", routes)
        .layer(CorsLayer::permissive())
        .layer(Extension(Arc::new(image_util)))
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await
}

pub async fn get_database_connection(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(config.timeout))
        .connect_lazy_with(config.with_db())
}
