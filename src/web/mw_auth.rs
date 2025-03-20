use crate::{Error, Result};
use axum::extract::Request; // Not axum::http::Request
use axum::middleware::{self, Next};
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(cookies: Cookies, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: Real auth-token parsing and validation

    let (user_id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

    Ok(next.run(req).await)
}

// Parse a token of format `user-[user-id].[exp].[sign]`
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenInvalid)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
