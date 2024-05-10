// return all the routes as /api/<route-path>

use axum::{
    routing::{get, post},
    Router,
};

use super::routes::{
    accept_file_upload, download_file, file_upload_form_new, get_file, health_check, ping_server,
    system_information,
};

// the app is moved here to allow sharing across test modules
pub fn app() -> Router {
    Router::new()
        .route("/", get(ping_server))
        .route(
            "/upload",
            post(accept_file_upload).get(file_upload_form_new),
        )
        .route("/health", post(accept_file_upload).get(health_check))
        .route("/api/sys-info", get(system_information))
        .route("/api/download", get(download_file))
        .route("/api/file", get(get_file))
}
