// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate uptime_lib;

use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::routing::{get, get_service, post};
use axum::Json;
use axum::Router;
use axum_typed_multipart::{FieldData, TempFile, TryFromMultipart, TypedMultipart};
use lazy_static::lazy_static;
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

use tower_http::services::ServeDir;

use crate::commands::utils::get_system_information;

// tauri APIs
use crate::commands::{
    audio::fetch_audio_files,
    documents::fetch_documents,
    send_file::share_file_with_peer,
    utils::{close_splashscreen, get_ip_address},
    video::fetch_video_files,
};

mod commands;
mod utils;
// uploaded file
//  data structure of a file uploaded to the recipient application core server
// provides the file metadata such as name, size, type and extension
#[derive(TryFromMultipart)]
struct UploadedFile {
    file: FieldData<TempFile>,
}

// allow sharing of the port
lazy_static! {
    pub static ref SERVER_PORT: u16 =
        portpicker::pick_unused_port().expect("failed to get an unused port");
}

fn main() {
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(core_server());

    println!("system information {}", get_system_information());
    // fire up tauri core
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            get_ip_address,
            fetch_audio_files,
            fetch_video_files,
            close_splashscreen,
            share_file_with_peer,
            get_system_information,
            fetch_documents
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
        .parse::<Ipv4Addr>().unwrap()
        ;
    let ip_address = format!("{:?}:{:?}", my_local_ip, *SERVER_PORT as u64);
    println!("server running on http://{}", &ip_address.to_string());

    //mount the application views
    let views_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("views");
    let static_files_service =
        get_service(ServeDir::new(views_dir).append_index_html_on_directories(true));

    // build our application with the required routes
    let app = Router::new()
        .fallback(static_files_service)
        .route("/upload", post(handle_file_upload))
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
async fn handle_file_upload(
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

// #[allow(clippy::let_and_return)]
// fn calling_serve_dir_from_a_handler() -> Router {
//     // via `tower::Service::call`, or more conveniently `tower::ServiceExt::oneshot` you can
//     // call `ServeDir` yourself from a handler
//     Router::new().nest_service(
//         "/foo",
//         get(|request: Request<Body>| async {
//             let service = ServeDir::new("assets");
//             let result = service.oneshot(request).await;
//             result
//         }),
//     )
// }
