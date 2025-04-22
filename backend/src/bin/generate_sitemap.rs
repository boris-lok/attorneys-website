use backend::domain;
use backend::domain::entities::{
    Language, Pagination, ResourceType, SimpleArticleEntity, SimpleMemberEntity,
};
use backend::get_configuration;
use backend::uow::InDatabase;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs::File;
use std::io::Write;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
struct Config {
    base_url: String,
    routes: Vec<String>,
}

fn get_config() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("src/configuration");

    let routes = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("routes.yaml"),
        ))
        .build()?;

    routes.try_deserialize::<Config>()
}

fn to_static_route(base_url: &str, s: &str) -> String {
    let full_url = format!("{}/{}", base_url, s);
    to_route(&full_url)
}

fn to_member_route(base_url: &str, id: &str) -> String {
    let full_url = format!("{}/members/{}", base_url, id);
    to_route(&full_url)
}

fn to_article_route(base_url: &str, id: &str) -> String {
    let full_url = format!("{}/articles/{}", base_url, id);
    to_route(&full_url)
}

fn to_route(s: &str) -> String {
    format!(
        "<url><loc>{}</loc><changefreq>weekly</changefreq></url>",
        s.trim_end_matches('/')
    )
}

fn generate_sitemap_string(
    base_url: &str,
    static_routes: Vec<String>,
    members: Vec<SimpleMemberEntity>,
    articles: Vec<SimpleArticleEntity>,
) -> anyhow::Result<String> {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    let total = 5 + members.len() + articles.len();

    let static_routes_string = static_routes
        .into_iter()
        .map(|e| to_static_route(base_url, &e))
        .chain(
            members
                .into_iter()
                .map(|e| to_member_route(base_url, &e.id)),
        )
        .chain(
            articles
                .into_iter()
                .map(|e| to_article_route(base_url, &e.id)),
        )
        .fold(String::with_capacity(256 * total), |init, e| {
            format!("{}{}", init, e)
        });
    xml.push_str(&static_routes_string);

    xml.push_str("</urlset>");

    Ok(xml)
}

async fn get_members(pool: &Pool<Postgres>) -> anyhow::Result<Vec<SimpleMemberEntity>> {
    let uow = InDatabase::new(pool).await?;
    let uow = Mutex::new(uow);

    let req = domain::resources::list::Request {
        filter_str: None,
        resource_type: ResourceType::Member,
        language: "zh".to_string(),
        default_language: Language::ZH,
        pagination: Pagination::All,
    };
    let (members, total) = domain::resources::list::execute::<_, SimpleMemberEntity>(uow, req)
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

    println!("got {} members", total);

    Ok(members)
}

async fn get_articles(pool: &Pool<Postgres>) -> anyhow::Result<Vec<SimpleArticleEntity>> {
    let uow = InDatabase::new(pool).await?;
    let uow = Mutex::new(uow);

    let req = domain::resources::list::Request {
        filter_str: None,
        resource_type: ResourceType::Article,
        language: "zh".to_string(),
        default_language: Language::ZH,
        pagination: Pagination::All,
    };
    let (articles, total) = domain::resources::list::execute::<_, SimpleArticleEntity>(uow, req)
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

    println!("got {} articles", total);

    Ok(articles)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let get_config = get_config()?;
    let base_url = get_config.base_url;
    let static_routes = get_config.routes;
    let configuration = get_configuration().expect("Can't get configuration");
    let database_connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(
            configuration.database.timeout,
        ))
        .connect_lazy_with(configuration.database.with_db());

    let members = get_members(&database_connection).await?;
    let articles = get_articles(&database_connection).await?;

    let xml = generate_sitemap_string(&base_url, static_routes, members, articles)?;

    let mut file = File::create("sitemap.xml")?;
    file.write_all(xml.as_bytes())?;

    println!("sitemap.xml generated ✅");
    Ok(())
}
