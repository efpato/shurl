mod auth;
pub mod db;
pub mod models;
pub mod routes;
pub mod shortener;
pub mod views;

#[derive(Clone, Debug)]
pub struct AppData {
    pub token: String,
}
