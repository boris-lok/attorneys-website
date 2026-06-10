use crate::api::routes::build_router;
use crate::configuration::{DatabaseSettings, Settings};
use crate::domain::services::resource::ResourceUoW;
use crate::domain::services::work_log::WorkLogUoW;
use crate::infrastructure::db::uow::PostgresUoWFactory;
use crate::utils::image::ImageUtil;
use jsonwebtoken::{DecodingKey, EncodingKey};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub upload_folder: String,
    pub jwt_encoding_key: Arc<EncodingKey>,
    pub jwt_decoding_key: Arc<DecodingKey>,
}

impl AppState {
    pub fn work_log_uow(&self) -> WorkLogUoW<PostgresUoWFactory> {
        WorkLogUoW::new(PostgresUoWFactory::new(self.pool.clone()))
    }

    pub fn resource_uow(&self) -> ResourceUoW<PostgresUoWFactory> {
        ResourceUoW::new(PostgresUoWFactory::new(self.pool.clone()))
    }
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
        upload_folder: config.application.upload_folder.to_string(),
        jwt_decoding_key,
        jwt_encoding_key,
    };
    let image_util = ImageUtil {};

    let app = build_router(state, redis_client, image_util);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

pub async fn get_database_connection(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(config.timeout))
        .connect_lazy_with(config.with_db())
}
