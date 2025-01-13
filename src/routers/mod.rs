// src/routers/mod.rs
pub mod news;  // Declare the news module here
pub mod download;
use axum::Router;

pub fn create_routes(service_name: &str) -> Router {
    Router::new().nest(&format!("/{}", service_name), news::create_news_router())
    .nest(&format!("/{}", service_name), download::create_download_handler())
}
