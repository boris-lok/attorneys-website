use backend::{get_configuration, get_subscriber, init_subscriber, run};
use std::fs::File;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tracing::log;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Can't get configuration");

    let writer: Mutex<Box<dyn std::io::Write + Send>> =
        match File::create(&configuration.settings.application.log_file) {
            Ok(file) => Mutex::new(Box::new(file)),
            Err(e) => {
                eprintln!(
                    "Failed to create log file '{}': {e}. Falling back to stdout.",
                    &configuration.settings.application.log_file
                );
                Mutex::new(Box::new(std::io::stdout()))
            }
        };
    let subscriber = get_subscriber(writer);
    init_subscriber(subscriber);
    log::info!("configuration: {:?}", &configuration);

    let address = format!(
        "{}:{}",
        &configuration.settings.application.host, &configuration.settings.application.port
    );
    let listener = TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {address} to TcpListener"));

    run(configuration, listener).await.unwrap()
}
