use anyhow::anyhow;
use backend::domain::users;
use backend::get_configuration;
use backend::repositories::{Connection, SqlxUserRepository};
use secrecy::SecretBox;
use sqlx::postgres::PgPoolOptions;
use std::io;
use std::io::Write;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration()?;
    let database_connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(
            configuration.database.timeout,
        ))
        .connect_lazy_with(configuration.database.with_db());
    let user_repo = SqlxUserRepository::new(Connection::Pool(database_connection));

    let mut reader = BufReader::new(stdin());
    let mut username = String::new();

    println!("Enter username: ");
    io::stdout().flush()?;

    reader.read_line(&mut username).await?;

    let mut password = String::new();
    println!("Enter password: ");
    io::stdout().flush()?;

    reader.read_line(&mut password).await?;

    let req = users::create_user::Request {
        username: username.trim().to_string(),
        password: SecretBox::new(Box::new(password.trim().to_string())),
    };

    match users::create_user::execute(req, Mutex::new(user_repo)).await {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("Failed to create user, got an error: {:?}", err)),
    }
}
