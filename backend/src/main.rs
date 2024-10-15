use backend::{get_configuration, run};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Can't get configuration");
    println!("{:?}", &configuration);

    let address = format!(
        "{}:{}",
        &configuration.application.host, &configuration.application.port
    );
    let listener = TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {address} to TcpListener"));

    run(configuration, listener).await.unwrap()
}
