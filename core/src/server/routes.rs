use axum::{BoxError, Json};
// get documents, audio, video, e.t.c and render the in the browser
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use futures::{Stream, TryStreamExt};
use hyper::header;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{self, BufWriter};
use tokio_util::io::{ReaderStream, StreamReader};

use axum::body::{Bytes, StreamBody};
use axum::extract::Multipart;
use serde_json::json;
use serde_json::Value;
use std::fs;

use crate::utils::{system_info::SystemInformation, CommandData};
use crate::UPLOAD_DIRECTORY;
// use axum::response::header;
// use http::header::{self, HeaderName};

// use axum::extract::DefaultBodyLimit;
// use axum::http::StatusCode;
// use axum::routing::{get, get_service, post};
// use axum::Json;
// use axum::Router;
// use futures::{Stream, TryStreamExt};
// use tower_http::services::ServeDir;
/// get the file path, return a file downloadable to the user
#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub file_path: String,
}
pub async fn download_file(Query(params): Query<Params>) -> impl IntoResponse {
    let Params { file_path } = params;
    let file = match tokio::fs::File::open(file_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let headers = [
        (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"Cargo.toml\"",
        ),
    ];

    Ok((headers, body))
}

/// return the system information
pub async fn system_information() -> (StatusCode, Json<CommandData<SystemInformation>>) {
    (
        StatusCode::OK,
        axum::Json(CommandData::ok(
            "connected system information ",
            SystemInformation::new(),
        )),
    )
}

// Handler that accepts a multipart form upload and streams each field to a file.
pub async fn accept_file_upload(
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Value>), (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        stream_to_file(&file_name, field).await?;
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "Success":true,
            "message":"file saved"
        })),
    ))
}

// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        //create wi-share directory in the downloads path dir and / save files to $DOWNLOADS/wi-share
        let os_default_downloads_dir = dirs::download_dir().unwrap();
        let upload_path = format!(
            "{downloads_dir}/{upload_dir}",
            downloads_dir = os_default_downloads_dir.display(),
            upload_dir = UPLOAD_DIRECTORY.as_str()
        );
        // create the uploads path if not exist
        let _ = fs::create_dir_all(&upload_path);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(&upload_path).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}

// 404 handler
pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::response::Json(serde_json::json!({
        "success":false,
        "message":String::from("The requested resource does not exist on this server!"),
        })),
    )
}

/// get the list of the audio files
pub async fn get_audio_files() {}

/// get the list of documents
pub async fn get_documents() {}

/// get the list of the video files
pub async fn get_video_files() {}

/// get the list of the image files
pub async fn get_image_files() {}

/// for a given file path, return the file the the used as a downloadable one
pub async fn get_file() {}
