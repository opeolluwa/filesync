// return all the routes as /api/<route-path>

use axum::{
    routing::{get, post},
    Router,
};

use super::routes::{
    accept_file_upload, download_file, get_audio_files, get_image_files, get_video_files,
    system_information,
};

// the app is moved here to allow sharing across test modules
pub fn app() -> Router {
    Router::new()
        .route("/upload", post(accept_file_upload))
        .route("/api/sys-info", get(system_information))
        .route("/api/download", get(download_file))
        .route("/api/document", get(download_file))
        .route("/api/images", get(get_image_files))
        .route("/api/videos", get(get_video_files))
        .route("/api/audio", get(get_audio_files))
}
