use axum::response::Html;
use axum::routing::get;
use axum::Router;
use local_ip_address::local_ip;
#[tokio::main]
async fn main() {
    let my_local_ip = local_ip().unwrap();
    let port: u16 = portpicker::pick_unused_port().expect("failed to get an unused port");
    let ip_address = format!("{:?}:{:?}", my_local_ip, port);

println!("server running on {ip_address:?}");

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    axum::Server::bind(&ip_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}