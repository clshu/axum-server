use crate::{Error, Result, web::AUTH_TOKEN};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

pub async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.email != "admin@test.com" || payload.password != "admin" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success response
    let body = Json(json!({
        "result": {
          "success": true,
        }
    }));

    Ok(body)
}
