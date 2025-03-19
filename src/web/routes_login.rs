use crate::{Error, Result};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

pub async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.email != "admin@test.com" || payload.password != "admin" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    // Create the success response
    let body = Json(json!({
        "result": {
          "success": true,
        }
    }));

    Ok(body)
}
