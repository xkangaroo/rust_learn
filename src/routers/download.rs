
use axum::{Router, routing::get};
use crate::handlers::download::{get_download_handler};

pub fn create_download_handler() -> Router {
    Router::new()
        .route("/download/{file_name}", get(get_download_handler))
}
