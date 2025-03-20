#![allow(unused)]

use axum::{
    Router,
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    serve::Serve,
};
use serde::Deserialize;
use tower_cookies::{CookieManager, CookieManagerLayer};
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Model Controller
    let mc = model::ModelConstroller::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        // layers are executed from bottom to top
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(get_service(ServeDir::new("./")));

    // region:    --- Start Server
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();

    // endregion: --- Start Server

    Ok(())
}

// region:    --- Response Mapper
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}
// endregion: --- Response Mapper

// region:    --- Route Hello

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

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

// e.g., `/hello2/Mike` i.e no query params /hello2/:name or /hello2/{name} in axum newer versions
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2: {name}", "HANDLER");

    Html(format!("Hello, <strong>{name}</strong>"))
}
// endregion: --- HAndlers
