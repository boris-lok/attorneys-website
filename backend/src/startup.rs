use crate::api::login::login;
use crate::api::logout::logout;
use crate::api::{
    create_article, create_contact, create_home, create_member, create_service, delete_article,
    delete_member, delete_service, health_check, list_articles, list_contact, list_home,
    list_members, list_services, retrieve_article, retrieve_contact, retrieve_home,
    retrieve_member, retrieve_service, update_article, update_contact, update_home, update_member,
    update_service, upload_member_avatar,
};
use crate::configuration::{DatabaseSettings, Settings};
use crate::utils::image::ImageUtil;
use axum::http::HeaderValue;
use axum::routing::{delete, get, post};
use axum::{Extension, Router};
use jsonwebtoken::{DecodingKey, EncodingKey};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub upload_folder: Arc<String>,
    pub jwt_encoding_key: Arc<EncodingKey>,
    pub jwt_decoding_key: Arc<DecodingKey>,
}

pub async fn run(config: Settings, listener: TcpListener) -> Result<(), std::io::Error> {
    let redis_client =
        redis::Client::open(config.redis_uri.as_str()).expect("Failed to connect the redis server");

    let jwt_encoding_key = Arc::new(EncodingKey::from_secret(
        config.application.jwt_secret.expose_secret().as_bytes(),
    ));
    let jwt_decoding_key = Arc::new(DecodingKey::from_secret(
        config.application.jwt_secret.expose_secret().as_bytes(),
    ));

    let state = AppState {
        pool: get_database_connection(&config.database).await,
        upload_folder: Arc::new(config.application.upload_folder),
        jwt_decoding_key,
        jwt_encoding_key,
    };
    let image_util = ImageUtil {};

    // Config the routes
    let admin_member_routes = Router::new()
        .route("/members", post(create_member).put(update_member))
        .route("/members/{id}", delete(delete_member))
        .route("/members/{id}/avatar", post(upload_member_avatar));
    let member_routes = Router::new()
        .route("/members/{id}", get(retrieve_member))
        .route("/members", get(list_members));

    let admin_service_routes = Router::new()
        .route("/services", post(create_service).put(update_service))
        .route("/services/{id}", delete(delete_service));
    let service_routes = Router::new()
        .route("/services/{id}", get(retrieve_service))
        .route("/services", get(list_services));

    let admin_home_routes = Router::new().route("/home", post(create_home).put(update_home));
    let home_routes = Router::new()
        .route("/home/{id}", get(retrieve_home))
        .route("/home", get(list_home));

    let admin_contact_routes =
        Router::new().route("/contact", post(create_contact).put(update_contact));
    let contact_routes = Router::new()
        .route("/contact/{id}", get(retrieve_contact))
        .route("/contact", get(list_contact));

    let admin_article_routes = Router::new()
        .route("/articles", post(create_article).put(update_article))
        .route("/articles/{id}", delete(delete_article));
    let article_routes = Router::new()
        .route("/articles/{id}", get(retrieve_article))
        .route("/articles", get(list_articles));

    let admin_user_routes = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));

    let admin_routes = Router::new()
        .merge(admin_member_routes)
        .merge(admin_home_routes)
        .merge(admin_service_routes)
        .merge(admin_contact_routes)
        .merge(admin_article_routes)
        .merge(admin_user_routes);

    let routes = Router::new()
        .merge(member_routes)
        .merge(service_routes)
        .merge(home_routes)
        .merge(contact_routes)
        .merge(article_routes);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/{version}/admin", admin_routes)
        .nest("/api/{version}/", routes)
        .layer(Extension(Arc::new(image_util)))
        .layer(Extension(Arc::new(redis_client)))
        .layer(CorsLayer::permissive())
        .layer(
            tower_http::set_header::response::SetResponseHeaderLayer::if_not_present(
                axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_static("*"),
            ),
        )
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await
}

pub async fn get_database_connection(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(config.timeout))
        .connect_lazy_with(config.with_db())
}
