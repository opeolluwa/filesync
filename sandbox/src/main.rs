use std::path::Path;

use axum::extract::DefaultBodyLimit;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use axum_typed_multipart::{
    FieldData, TempFile, TryFromMultipart, TypedMultipart,
};
use local_ip_address::local_ip;
use serde_json::json;
use serde_json::Value;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// uploaded file
/// represent file that is uploaded to application core server
#[derive(TryFromMultipart)]
struct UploadedFile {
    file: FieldData<TempFile>,
}

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

    // define cors scope
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

// handle file upload manually
// retrieve the file name
// the file type
// and stream the content to a binary data
async fn recieve_files(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{}` (`{}`: `{}` {}) is bytes",
            name,
            file_name,
            content_type,
            compute_file_size(data.len().try_into().unwrap())
        );
    }
    (
        StatusCode::OK,
        Json(json!({
            "Success":true
        })),
    )
}

// a function to compute file size
// accept files size in byte and parse it to human readable KB, MB, TB, GB e.t.c
pub fn compute_file_size(size: u64) -> String {
    if size > (1024 * 1024 * 1024 * 1024) {
        return format!("{:.2} TB", size / (1024 * 1024 * 1024 * 1024));
    } else if size > (1024 * 1024 * 1024) {
        return format!("{:.2} GB", size / (1024 * 1024 * 1024));
    } else if size > (1024 * 1024) {
        return format!("{:.2} MB", size / (1024 * 1024));
    } else if size > 1024 {
        return format!("{:.2} KB", size / (1024));
    } else {
        return format!("{:.2} B", size);
    }
}

/// handle file upload with typed header
async fn handle_file_upload(
    TypedMultipart(UploadedFile { file }): TypedMultipart<UploadedFile>,
) -> (StatusCode, Json<Value>) {
    let file_name = file.metadata.file_name.unwrap_or(String::from("data.bin"));

    // save the file to download dir of the operating systems
    let download_dir = dirs::download_dir().unwrap();
    println!("download dir! {download_dir:?}");
    let path = Path::new(&download_dir).join(file_name);

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
