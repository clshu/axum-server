#![allow(unused)]

use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let route_hello = Router::new().route("/hello", get(handler_hello));
    // .route("/hello2/:name", get(handler_hello2));

    // region:    --- Start Server
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, route_hello).await.unwrap();
    // endregion: --- Start Server
}

// region:    --- HAndlers
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=Bob`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello, <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
// async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
//     println!("->> {:<12} - handler_hello: {name}", "HANDLE ");

//     Html(format!("Hello, <strong>{name}</strong>"))
// }
// endregion: --- HAndlers
