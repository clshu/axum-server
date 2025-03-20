use crate::{Error, Result};
use axum::extract::Request; // Not axum::http::Request
use axum::middleware::{self, Next};
use axum::response::Response;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(cookies: Cookies, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: Real auth-token parsing and validation

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}
