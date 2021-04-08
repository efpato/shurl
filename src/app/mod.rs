mod auth;
pub mod config;
pub mod db;
pub mod models;
pub mod shortener;
pub mod views;

#[derive(Clone, Debug)]
pub struct AppData {
    pub token: String,
}
