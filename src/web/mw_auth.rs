use crate::ctx::Ctx;
use crate::{Error, Result};
use axum::extract::Request; // Not axum::http::Request
use axum::middleware::{self, Next};
use axum::response::Response;

use crate::web::AUTH_TOKEN;

// ctx: Result<Ctx> is the custom extractor, ned to call ctx?; in the middleware
// ctx: Ctx, the middleware is not called, comment out ctx? in the middleware

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    // When the custom extractor extracts ctx, it performs the auth check
    ctx?;

    Ok(next.run(req).await)
}
