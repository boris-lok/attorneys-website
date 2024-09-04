use backend::{get_configuration, run};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0";
    let port = 1234;

    let configuration = get_configuration().expect("Can't get configuration");

    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {address} to TcpListener"));

    run(configuration, listener).await.unwrap()
}
