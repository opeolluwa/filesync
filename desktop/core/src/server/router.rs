// return all the routes as /api/<route-path>

use axum::{
    routing::{get, post},
    Router,
};

use memory_serve::{load_assets, MemoryServe};

use super::routes::{
    accept_file_upload, get_file, handle_404, health_check, notify_peer, system_information,
};

// the app is moved here to allow sharing across test modules
pub fn app() -> Router {
    let memory_router: axum::routing::Router<()> = MemoryServe::new(load_assets!("static"))
        .index_file(Some("/index.html"))
        .into_router();

    Router::new()
        // .route("/upload", post(accept_file_upload))
        // .route("/health", post(accept_file_upload).get(health_check))
        // .route("/api/sys-info", get(system_information))
        // .route("/api/file", get(get_file))
        .route("/notify", get(notify_peer))
        // .merge(memory_router)
        // .fallback(handle_404)
}
