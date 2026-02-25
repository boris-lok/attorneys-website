use anyhow::anyhow;
use backend::domain::entities::UserID;
use backend::domain::users;
use backend::get_configuration;
use backend::repositories::{
    IRolesRepository, SqlxRolesRepository, SqlxUserRepository, SqlxUserRolesRepository,
};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, MultiSelect, Password};
use secrecy::SecretBox;
use sqlx::postgres::PgPoolOptions;
use sqlx::Acquire;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse command line arguments
    let cli = Cli::parse();

    // get the configuration and make a connection with the database
    let configuration = get_configuration()?;
    let conn = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(
            configuration.database.timeout,
        ))
        .connect_lazy_with(configuration.database.with_db());
    let conn = Arc::new(conn);

    match cli.commands {
        Commands::List => list_users(conn).await?,
        Commands::Create { username, nickname } => {
            create_user(conn.clone(), username, nickname).await?
        }
        Commands::Delete { id } => delete_user(conn.clone(), id).await?,
    }

    Ok(())
}

#[derive(clap::Parser)]
#[command(version, propagate_version = true)]
#[command(subcommand_required = true, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// list all users
    List,
    /// create a user
    Create {
        /// The username of the user
        #[arg(long)]
        username: String,
        /// The nickname of the user, use for the report
        #[arg(long)]
        nickname: String,
    },
    /// delete a user
    Delete {
        /// The ID of the user to delete
        #[arg(long)]
        id: String,
    },
}

/// list all users from the database and print the result in the console
async fn list_users(pool: Arc<sqlx::PgPool>) -> anyhow::Result<()> {
    let conn = &mut *pool.acquire().await?;
    let user_repo = SqlxUserRepository::new(Arc::new(Mutex::new(conn)));

    let res = users::list_users::execute(Mutex::new(user_repo))
        .await
        .map_err(|e| anyhow!("Failed to list users, got an error: {:?}", e))?;

    if res.is_empty() {
        println!("No users found");
        return Ok(());
    }

    for user in res {
        println!(
            "ID: {}, username: {}, nickname: {}, roles: {}",
            user.id,
            user.username,
            user.nickname,
            user.roles.join(", ")
        );
    }

    Ok(())
}

async fn delete_user(pool: Arc<sqlx::PgPool>, id: String) -> anyhow::Result<()> {
    let user_id = UserID::try_from(id).map_err(|_| anyhow!("Invalid user ID, must be a UUID"))?;

    let conn = &mut *pool.acquire().await?;
    let user_repo = SqlxUserRepository::new(Arc::new(Mutex::new(conn)));

    users::delete_user::execute(
        users::delete_user::Request { id: user_id },
        Mutex::new(user_repo),
    )
    .await?;

    println!("User deleted successfully");

    Ok(())
}

async fn create_user(
    pool: Arc<sqlx::PgPool>,
    username: String,
    nickname: String,
) -> anyhow::Result<()> {
    let conn = &mut *pool.acquire().await?;

    let role_repo = SqlxRolesRepository::new(Mutex::new(conn));

    let roles = role_repo.list_roles().await;

    let selection = roles
        .clone()
        .into_iter()
        .map(|role| role.name)
        .collect::<Vec<_>>();

    let selected_roles = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select roles")
        .items(&selection[..])
        .interact()?;

    if selected_roles.is_empty() {
        println!("No roles selected :(");
        return Ok(());
    }

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter password")
        .with_confirmation("Repeat password", "Error: the passwords do not match")
        .validate_with(|password: &String| {
            if password.chars().count() < 7 {
                return Err("Password must be at least 8 characters long".to_string());
            }

            Ok(())
        })
        .interact()?;

    let secret_password = SecretBox::new(Box::new(password));
    let selected_roles = selected_roles
        .iter()
        .map(|index| roles[*index].id)
        .collect::<Vec<_>>();

    let mut tx = pool.begin().await?;
    let conn = tx.acquire().await?;
    let conn = Arc::new(Mutex::new(conn));

    let user_repo = SqlxUserRepository::new(conn.clone());
    let user_role_repo = SqlxUserRolesRepository::new(conn);

    let req = users::create_user::Request {
        username: username.clone(),
        password: secret_password,
        nickname: nickname.clone(),
        role_ids: selected_roles,
    };

    let user_id =
        users::create_user::execute(req, Mutex::new(user_repo), Mutex::new(user_role_repo))
            .await
            .map_err(|e| anyhow!("Failed to create user, got an error: {:?}", e))?;

    println!("User created with ID: {}", user_id);

    tx.commit().await?;

    Ok(())
}
