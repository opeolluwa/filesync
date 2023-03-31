use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum::extract::Multipart
use local_ip_address::local_ip;

#[tokio::main]
async fn main() {
    let my_local_ip = local_ip().unwrap();
    let port: u16 = portpicker::pick_unused_port().expect("failed to get an unused port");
    let ip_address = format!("{:?}:{:?}", my_local_ip, port);

    println!("server running on {ip_address:?}");
    let file_limit = RequestBodyLimitLayer::new(10 * 1024 * 1024 * 1024);

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/files", post(recieve_files))
        .layer(file_limit);

    // run it
    axum::Server::bind(&ip_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(
        r#"
        <h1> sample upload </h1>
     <form action="http://localhost:3000/upload" method="post" enctype="multipart/form-data">
        <label>
            Upload file:
            <input type="file" name="file" multiple>
        </label>
        <input type="submit" value="Upload files">
    </form>
    "#,
    )
}

async fn recieve_files(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        // let file_name = field.file_name().unwrap().to_string();
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
