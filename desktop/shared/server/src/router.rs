// return all the routes as /api/<route-path>
use axum::{
    routing::{get, post},
    Router,
};

use super::routes::{accept_file_upload, get_file, handle_404, health_check};

// use memory_serve::{load_assets, MemoryServe};

// the app is moved here to allow sharing across test modules
pub fn app() -> Router {
    // let static_files: axum::routing::Router<()> = MemoryServe::new(load_assets!("static"))
    //     .index_file(Some("/index.html"))
    //     .into_router();

    Router::new()
        .route("/upload", post(accept_file_upload))
        .route("/health", post(accept_file_upload).get(health_check))
        .route("/api/file", get(get_file))
        // .route("/notify", get(notify_peer))
        // .merge(static_files)
        .fallback(handle_404)
}
