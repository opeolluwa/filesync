use reqwest::Method;

use std::path::PathBuf;

use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use axum::extract::DefaultBodyLimit;
use axum::routing::get_service;

use tower_http::services::ServeDir;

use crate::SERVER_PORT;

mod router;
mod routes;
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
        .expect("invalid socket address");

    tracing::debug!("server running on http://{}", &ip_address.to_string());

    //mount the application views
    let views_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("views");
    let static_files_service =
        get_service(ServeDir::new(views_dir).append_index_html_on_directories(true));

    // build our application with the required routes
    let app = router::app()
        .fallback(static_files_service)
        .layer(file_limit)
        .layer(cors_layer)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // add a fallback service for handling routes to unknown paths
    // let app = app.fallback(handle_404);

    // run the server
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
        let app = router::app();

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
        let app = router::app();

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
