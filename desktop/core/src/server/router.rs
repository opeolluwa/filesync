// return all the routes as /api/<route-path>

use std::path::PathBuf;

use axum::{
    routing::{get, post},
    Router,
};
use hyper::{Body, Request};
use tower::ServiceExt;
use tower_http::services::ServeDir;

use super::routes::{
    accept_file_upload, download_file, file_upload_form, get_file, handle_404, health_check,
    notify_peer, ping_server, system_information,
};

// the app is moved here to allow sharing across test modules
pub fn app() -> Router {
    Router::new()
        .route("/", get(ping_server))
        .route("/upload", post(accept_file_upload))
        .route("/health", post(accept_file_upload).get(health_check))
        .route("/api/sys-info", get(system_information))
        .route("/api/download", get(download_file))
        .route("/api/file", get(get_file))
        .route("/notify", get(notify_peer))
        .nest_service(
            "/view",
            get(|request: Request<Body>| async {
                ServeDir::new("../views").oneshot(request).await
            }),
        )
        .fallback(handle_404)
}
