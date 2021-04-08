#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use pretty_env_logger::env_logger;

mod app;
mod schema;

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let app_host = std::env::var("APP_HOST").expect("APP_HOST not found");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT not found");
    let username = std::env::var("APP_USER").expect("APP_USER not found");
    let password = std::env::var("APP_PASS").expect("APP_PASS not found");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

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
            .configure(app::config::config)
    })
    .bind(format!("{}:{}", app_host, app_port))?
    .run()
    .await
}
