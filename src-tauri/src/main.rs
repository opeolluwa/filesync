// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use axum::{
    extract::Multipart,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tauri::Manager;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod commands;


#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tokio::main]
async fn main() {
    // let aud_files = commands::fetch_audio_files().ok().unwrap();
    // println!("the audio files {:?}", aud_files.data.unwrap()[6]);

    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "send_file_core=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // define cors scope
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    // define file limit layer as 10GB
    let file_limit = RequestBodyLimitLayer::new(10 * 1024 * 1024 * 1024);
    // build our application with the required routes
    // the index route for debugging
    // and the upload route for file upload
    let app = Router::new()
        .route("/upload", post(recieve_files))
        .route("/", get(index))
        .layer(file_limit)
        .layer(cors_layer);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let port = portpicker::pick_unused_port().expect("failed to get an unused port");
    let ip_address = SocketAddr::from(([0, 0, 0, 0], port));

    //launch the server on a parallel process
    println!("Ignition started dd on http://{}", &ip_address);
    /*  axum::Server::bind(&ip_address)
    .serve(app.into_make_service())
    .await
    .unwrap() */
   

    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_ip_addr,
            commands::audio::fetch_audio_files,
            commands::video::fetch_video_files,
            close_splashscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

     tokio::task::spawn(
        axum::Server::bind(&ip_address).serve(app.into_make_service()), /*  .await
                                                                        .unwrap() */
    );
}

//recieve a file
// save the file to users/download/send-file
// to do this
// see if folder exists
// create if not
async fn recieve_files(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{}` (`{}`: `{}`) is {} bytes",
            name,
            file_name,
            content_type,
            data.len()
        );
    }
}

// basic handler that responds with a static string
async fn index() -> &'static str {
    "Hello, World!"
}
