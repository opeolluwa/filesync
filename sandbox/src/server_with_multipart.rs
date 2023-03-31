use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use local_ip_address::local_ip;
use serde_json::json;

#[tokio::main]
async fn main() {
    let my_local_ip = local_ip().unwrap();
    let port: u16 = portpicker::pick_unused_port().expect("failed to get an unused port");
    let ip_address = format!("{:?}:{:?}", my_local_ip, port);
    println!("server running on {:?}", ip_address);

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/upload", post(recieve_files));

    // run it
    axum::Server::bind(&ip_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<String> {
    Html(format!(
        "
   <html>

<head></head>

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
    (
        StatusCode::OK,
        Json(json!({
            "Success":true
        })),
    )
}
