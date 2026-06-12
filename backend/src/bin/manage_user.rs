use anyhow::anyhow;
use backend::domain::role::repository::RoleRepository;
use backend::domain::uow::user::UserUoW;
use backend::domain::users;
use backend::domain::users::entity::UserID;
use backend::get_configuration;
use backend::infrastructure::db::connection::{PostgresRepo, RoleRepo, UserRepo};
use backend::infrastructure::db::uow::{PostgresQuery, PostgresUoWFactory};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, MultiSelect, Password};
use secrecy::SecretBox;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse command line arguments
    let cli = Cli::parse();

    // get the configuration and make a connection with the database
    let configuration = get_configuration()?;
    let conn = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(
            configuration.settings.database.timeout,
        ))
        .connect_lazy_with(configuration.settings.database.with_db());

    match cli.commands {
        Commands::List => list_users(conn).await?,
        Commands::Create { username, nickname } => create_user(conn, username, nickname).await?,
        Commands::Delete { id } => delete_user(conn, id).await?,
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
async fn list_users(pool: sqlx::PgPool) -> anyhow::Result<()> {
    let query = PostgresQuery::new(pool);

    let res = users::list::execute(&query)
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

async fn delete_user(pool: sqlx::PgPool, id: String) -> anyhow::Result<()> {
    let mut repo = PostgresRepo::<UserRepo>::from_pool(&pool);

    let user_id = UserID::try_from(id).map_err(|_| anyhow!("Invalid user ID, must be a UUID"))?;

    users::delete::execute(&mut repo, &user_id).await?;

    println!("User deleted successfully");

    Ok(())
}

async fn create_user(pool: sqlx::PgPool, username: String, nickname: String) -> anyhow::Result<()> {
    let mut role_repo = PostgresRepo::<RoleRepo>::from_pool(&pool);

    let roles = role_repo.list().await;

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

    let service = UserUoW::new(PostgresUoWFactory::new(pool));

    let req = users::create::Request {
        username: username.clone(),
        password: secret_password,
        nickname: nickname.clone(),
        role_ids: selected_roles,
    };

    let user_id = users::create::execute(&service, req)
        .await
        .map_err(|e| anyhow!("Failed to create user, got an error: {:?}", e))?;

    println!("User created with ID: {}", user_id);

    Ok(())
}
