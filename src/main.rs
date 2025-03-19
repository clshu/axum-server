#![allow(unused)]

use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let route_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello, <strong>World!</strong>") }),
    );

    // region:    --- Start Server
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, route_hello).await.unwrap();
    // endregion: --- Start Server
}
