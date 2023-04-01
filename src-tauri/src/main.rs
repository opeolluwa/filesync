// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(const_option)]
// #[allow(unused_variables)]

use std::thread;
use std::{fs, path::Path};

use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use axum_typed_multipart::{FieldData, TempFile, TryFromMultipart, TypedMultipart};
use local_ip_address::local_ip;
use once_cell::sync::Lazy;
use serde_json::json;
use serde_json::Value;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::commands::send_file::share_file_with_peer;
// tauri APIs
use crate::commands::{
    audio::fetch_audio_files,
    utils::{close_splashscreen, get_ip_addr},
    video::fetch_video_files,
};

mod commands;

// uploaded file
// represent file that is uploaded to application core server
// also let use access the file metadata such as name, size, type and extension
#[derive(TryFromMultipart)]
struct UploadedFile {
    file: FieldData<TempFile>,
}

// assign a port to the application core
pub static SERVER_PORT: Lazy<u16> =
    Lazy::new(|| portpicker::pick_unused_port().expect("failed to get an unused port"));

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "send_file_core=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // define cors scope as any
    // change this later to only allow get and post http verbs
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    // define file limit layer as 10GB
    // see information here <https://docs.rs/axum/0.6.2/axum/extract/struct.DefaultBodyLimit.html#%E2%80%A6>
    let file_limit = DefaultBodyLimit::max(10 * 1024 * 1024 * 1024);

    let my_local_ip = local_ip().unwrap();
    let port: u16 = portpicker::pick_unused_port().expect("failed to get an unused port");
    let ip_address = format!("{:?}:{:?}", my_local_ip, port);
    println!("server running on http://{:?}", ip_address);

    // initialize the tauri app here
    // tokio::task::spawn(init_tauri());
    // init_tauri();
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            get_ip_addr,
            fetch_audio_files,
            fetch_video_files,
            close_splashscreen,
            share_file_with_peer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // let tauri_handler = thread::spawn(move || init_tauri());

    // build our application with the required routes
    // the index route for debugging
    // and the upload route for file upload
    let app = Router::new()
        .route("/", get(handler))
        .route("/upload", post(handle_file_upload))
        .layer(file_limit)
        .layer(cors_layer)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // run it
    let core_server = thread::spawn(move || async move {
        axum::Server::bind(&ip_address.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // let core_server =
    core_server.join().unwrap();
}

async fn handler() -> Html<String> {
    Html(format!(
        "
         <!doctype html>
   <html>
<head>

</head>
<body>
<h1> hey man </h1>
    <form action='/upload' method='post' enctype='multipart/form-data'>
        <label>
            Upload file:
            <input type='file' name='file' multiple>
        </label>
        <input type='submit' value='Upload files'>
    </form>
</body>

</html>
   "
    ))
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

// fire up the tauri server
fn _init_tauri() {
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            get_ip_addr,
            fetch_audio_files,
            fetch_video_files,
            close_splashscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // println!("hey famz");
}
