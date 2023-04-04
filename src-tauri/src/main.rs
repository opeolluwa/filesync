// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//#![feature(const_option)]
// #[allow(unused_variables)]

// #[macro_use]
// extern crate lazy_static;
// use std::thread;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use axum_typed_multipart::{FieldData, TempFile, TryFromMultipart, TypedMultipart};
use lazy_static::lazy_static;
use local_ip_address::local_ip;
use serde_json::json;
use serde_json::Value;
use std::{fs, path::Path};
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// use crate::commands::send_file::share_file_with_peer;
// tauri APIs
use crate::commands::{
    audio::fetch_audio_files,
    send_file::share_file_with_peer,
    utils::{close_splashscreen, get_ip_address},
    video::fetch_video_files,
};

mod commands;
//mod config;
// uploaded file
// represent file that is uploaded to application core server
// also let use access the file metadata such as name, size, type and extension
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
    // plug the server
    tauri::async_runtime::spawn(core_server());
    tauri::async_runtime::spawn(share_file_with_peer(
        "/home/drizzle/Documents/download/Childish_Gambino_-_Sober(256k).mp3".to_string(),
        *SERVER_PORT,
    ));
    println!("ip {}", *SERVER_PORT); 

    // fire up tauri core
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            get_ip_address,
            fetch_audio_files,
            fetch_video_files,
            close_splashscreen,
            share_file_with_peer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn handler() -> Html<String> {
    Html(
        r#"
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
   "#
        .to_string(),
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

pub async fn core_server() {
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
    let ip_address = format!("{:?}:{:?}", my_local_ip, *SERVER_PORT);
    println!("server running on http://{:?}", ip_address);

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
    axum::Server::bind(&ip_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    // let core_server =
}
