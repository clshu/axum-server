#![allow(unused)]

use axum::{
    Json, Router,
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    serve::Serve,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{CookieManager, CookieManagerLayer};
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod model;
mod web;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Model Controller
    let mc = model::ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        // layers are executed from bottom to top
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
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
    let uuid = Uuid::new_v4();

    // Get the error from the response
    let service_error = res.extensions().get::<Error>();
    // println!(
    //     "->> {:<12} - service_error: {:?}",
    //     "RES_MAPPER", service_error
    // );
    let client_status_error = service_error.map(|e| e.cinet_status_and_error());
    // println!(
    //     "->> {:<12} - client_error: {:?}",
    //     "RES_MAPPER", client_status_error
    // );

    // If client error, create a response with the error
    let error_response = client_status_error.as_ref().map(|(status, client_error)| {
        let client_error_body = json!(
        { "error": {
            "type": client_error.as_ref(),
            "req_uuid": uuid.to_string(),
        }});
        println!("->> {:<12} - {client_error_body}", "RES_MAPPER");

        (*status, Json(client_error_body)).into_response()
    });
    // println!(
    //     "->> {:<12} - error_response: {:?}",
    //     "RES_MAPPER", error_response
    // );
    println!();
    error_response.unwrap_or(res)
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
