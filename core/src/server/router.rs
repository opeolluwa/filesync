// return all the routes as /api/<route-path>

use axum::{
    routing::{get, post},
    Router,
};

use super::routes::{accept_file_upload, download_file, system_information};

// the app is moved here to allow sharing across test modules
pub fn app() -> Router {
    Router::new()
        .route("/upload", post(accept_file_upload))
        .route("/sys-info", get(system_information))
        .route("/download", get(download_file))
}
