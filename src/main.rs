#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use pretty_env_logger::env_logger;
use std::env;

mod app;
mod schema;

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to read .env file");

    env::set_var("RUST_LOG", "shurl=info");
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_addr = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = app::db::create_db_pool(&db_url);
    let shortener = app::shortener::Shortener::new();

    info!("Serving on {}", app_addr);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(shortener.clone())
            .configure(app::config::config)
    })
    .bind(app_addr)?
    .run()
    .await
}
