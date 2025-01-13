
use axum::{Router, routing::get};
use crate::handlers::news::{get_news_handler, post_news_handler};

pub fn create_news_router() -> Router {
    Router::new()
        .route("/news", get(get_news_handler))
        .route("/news/Q", get(post_news_handler))
}
