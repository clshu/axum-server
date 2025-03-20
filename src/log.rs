use std::alloc::System;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ctx::{self, Ctx};
use crate::{ClientError, Error, Result};

use axum::http::{Method, Uri};
use serde::{Serialize, de};
use serde_json::{Value, json};
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestampe = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.map(|e| e.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|d| d.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestampe.to_string(),
        user_id: ctx.map(|c| c.user_id()),
        req_path: uri.path().to_string(),
        req_method: req_method.to_string(),
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    println!(
        "  ->> log_line: user_id {:?}\n{}",
        log_line.user_id,
        json!(log_line)
    );

    // Send log_line to logging service

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String, // ISO 8601

    // -- User and context attributes
    user_id: Option<u64>,

    // -- Request attributes
    req_path: String,
    req_method: String,

    // -- Error attributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
