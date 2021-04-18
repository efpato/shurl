#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use pretty_env_logger::env_logger;
use serde::Deserialize;

mod app;
mod schema;

#[derive(Deserialize)]
struct Config {
    app_host: String,
    app_port: u16,
    app_user: String,
    app_pass: String,
    db_url: String,
    log_level: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let content = std::fs::read_to_string("configs/default.toml").expect("Config file not found!");
    let config: Config = toml::from_str(content.as_str()).expect("Couldn't parse config file!");

    let app_host = std::env::var("APP_HOST").unwrap_or(config.app_host);
    let app_port = std::env::var("APP_PORT").unwrap_or(config.app_port.to_string());
    let username = std::env::var("APP_USER").unwrap_or(config.app_user);
    let password = std::env::var("APP_PASS").unwrap_or(config.app_pass);
    let db_url = std::env::var("DATABASE_URL").unwrap_or(config.db_url);
    let log_level = std::env::var("LOG_LEVEL").unwrap_or(config.log_level);

    std::env::set_var("RUST_LOG", log_level);
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let app_data = app::AppData {
        token: base64::encode(format!("{}:{}", username, password)),
    };

    let pool = app::db::create_db_pool(&db_url);
    let shortener = app::shortener::Shortener::new();

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .data(pool.clone())
            .data(shortener.clone())
            .wrap(middleware::Logger::default())
            .configure(app::routes::init_routes)
    })
    .bind(format!("{}:{}", app_host, app_port))?
    .run()
    .await
}
