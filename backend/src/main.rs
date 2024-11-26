use backend::{get_configuration, get_subscriber, init_subscriber, run};
use std::fs::File;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tracing::log;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Can't get configuration");

    let file = File::create(&configuration.application.log_file).unwrap();
    let subscriber = get_subscriber(Mutex::new(file));
    init_subscriber(subscriber);
    log::info!("configuration: {:?}", &configuration);

    let address = format!(
        "{}:{}",
        &configuration.application.host, &configuration.application.port
    );
    let listener = TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {address} to TcpListener"));

    run(configuration, listener).await.unwrap()
}
