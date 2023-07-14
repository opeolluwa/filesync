use axum::body::Bytes;
use axum::extract::Multipart;
use axum::response::IntoResponse;
use axum::BoxError;
use reqwest::Method;
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utils::system_info::SystemInformation;
use utils::CommandData;

use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::routing::{get, get_service, post};
use axum::Json;
use axum::Router;
use futures::{Stream, TryStreamExt};
use tower_http::services::ServeDir;

use std::io;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;

use crate::utils;
use crate::{SERVER_PORT, UPLOAD_DIRECTORY};

/**
 * @function core_server
 * the application core responsible for handling file upload to client
 *  machine and file download to the host machine
 */
pub async fn core_server() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "send_file_core=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init(); // allow debugging in development set up

    // define cors scope as any
    // change this later to only allow get and post http verbs
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST]) // restrict methods
        .allow_origin(Any); // TODO: restrict this in the future to only sendfile proxy server for example http://sendfile/dhsdo

    // define file limit layer as 10GB
    // see information here <https://docs.rs/axum/0.6.2/axum/extract/struct.DefaultBodyLimit.html#%E2%80%A6>
    let file_size_limit = 10 * 1024 * 1024 * 1024;
    let file_limit = DefaultBodyLimit::max(file_size_limit);

    // TODO: run the https server on localhost then feed off the connection using the wifi gateway, the same way Vite/Vue CLI would do the core server
    // this is currently achieved by binding the server to the device default ip address
    let my_local_ip = crate::net::ip_manager::autodetect_ip_address()
        .expect("No Ip address detected")
        .parse::<std::net::Ipv4Addr>()
        .unwrap();
    let ip_address = format!("{:?}:{:?}", my_local_ip, *SERVER_PORT as u64);
    let ip_address = ip_address
        .parse::<std::net::SocketAddr>()
        .ok()
        .expect("invalid socket address");

    tracing::debug!("server running on http://{}", &ip_address.to_string());

    //mount the application views
    let views_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("views");
    let static_files_service =
        get_service(ServeDir::new(views_dir).append_index_html_on_directories(true));

    // build our application with the required routes
    let app = app()
        .fallback(static_files_service)
        .layer(file_limit)
        .layer(cors_layer)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handle_404);

    // run the server
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// return the system information
async fn system_information() -> (StatusCode, Json<CommandData<SystemInformation>>) {
    (
        StatusCode::OK,
        axum::Json(CommandData::ok(
            "connected system information ",
            SystemInformation::new(),
        )),
    )
}

// Handler that accepts a multipart form upload and streams each field to a file.
async fn accept_file_upload(
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
async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::response::Json(serde_json::json!({
        "success":false,
        "message":String::from("The requested resource does not exist on this server!"),
        })),
    )
}

// the app is moved here to allow sharing across test modules
pub fn app() -> Router {
    Router::new()
        .route("/upload", post(accept_file_upload))
        .route("/sys-info", get(system_information))
}

#[cfg(test)]
mod basic_endpoints {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    // use serde_json::{json, Value};
    use tower::ServiceExt;
    // test the server base url
    // for example ->  http://loccalhost:4835
    // the index route should return hello world
    #[tokio::test]
    async fn base_url() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // response status code should be 200
        assert_eq!(response.status(), StatusCode::OK);
    }

    // 404 path
    #[tokio::test]
    async fn not_found_handler() {
        let app = app();

        // the 404 handle should return this json
        // it will return a NOT_FOUND  status code
        // the test will test for the validity of  this.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/not-found-error")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // assert  the the status code is 404
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        // println!(" the not-found-endpoint response is {response:?}");
    }
}
