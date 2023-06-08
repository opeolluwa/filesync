use axum::body::Bytes;
use axum::extract::Multipart;
use axum::BoxError;
use reqwest::Method;
use serde_json::json;
use serde_json::Value;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::{fs, path::Path};

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

use axum_typed_multipart::{FieldData, TempFile, TryFromMultipart, TypedMultipart};

use crate::utils;
use crate::SERVER_PORT;

// uploaded file
//  data structure of a file uploaded to the recipient application core server
// provides the file metadata such as name, size, type and extension
#[derive(TryFromMultipart)]
struct UploadedFile {
    file: FieldData<TempFile>,
}

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

    // the core server
    let my_local_ip = utils::ip_manager::autodetect_ip_address()
        .ok()
        .expect("Invalid Ip address detected")
        .parse::<Ipv4Addr>()
        .unwrap();
    let ip_address = format!("{:?}:{:?}", my_local_ip, *SERVER_PORT as u64);
    debug!("server running on http://{}", &ip_address.to_string());

    //mount the application views
    let views_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("views");
    let static_files_service =
        get_service(ServeDir::new(views_dir).append_index_html_on_directories(true));

    // build our application with the required routes
    let app = Router::new()
        .fallback(static_files_service)
        .route("/upload", post(accept_file_upload))
        .route("/sys-info", get(system_information))
        .layer(file_limit)
        .layer(cors_layer)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // run the server
    axum::Server::bind(&ip_address.parse().unwrap())
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

/// handle file upload with typed header
async fn _handle_file_upload(
    TypedMultipart(UploadedFile { file }): TypedMultipart<UploadedFile>,
) -> (StatusCode, Json<Value>) {
    // save the file to download dir of the operating systems
    // println!("download dir! {download_dir:?}");
    //create send-file directory in the downloads path dir
    let file_name = file.metadata.file_name.unwrap_or(String::from("data.bin"));
    let os_default_downloads_dir = dirs::download_dir().unwrap();
    /*  let Some(os_default_downloads_dir ) = dirs::download_dir() else{
        return  Err(error_message) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "Success":false,
                "message": error_message.to_string()
            })),
        );
    } */
    // save files to $DOWNLOADS/send-file
    let upload_path = format!(
        "{downloads_dir}/send-file",
        downloads_dir = os_default_downloads_dir.display()
    );
    // create the uploads path if not exist
    let _ = fs::create_dir_all(&upload_path);
    let path = Path::new(&upload_path).join(file_name);

    match file.contents.persist(path, false).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "Success":true,
                "message":"file saved"
            })),
        ),
        Err(error_message) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "Success":false,
                "message": error_message.to_string()
            })),
        ),
    }
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

        //create send-file directory in the downloads path dir and / save files to $DOWNLOADS/send-file
        let os_default_downloads_dir = dirs::download_dir().unwrap();
        let upload_path = format!(
            "{downloads_dir}/send-file",
            downloads_dir = os_default_downloads_dir.display()
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
