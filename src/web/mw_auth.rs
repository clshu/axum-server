use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::extract::FromRequestParts;
use axum::extract::{Request, State}; // Not axum::http::Request

use axum::http::request::Parts;
use axum::middleware::{self, Next};
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

// ctx: Result<Ctx> is the custom extractor, ned to call ctx?; in the middleware
// ctx: Ctx, the middleware is not called, comment out ctx? in the middleware

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    // When the custom extractor extracts ctx, it performs the auth check
    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, exp, sign)) => Ok(Ctx::new(user_id)),
        Err(e) => Err(e),
    };

    // Remove cookie if something went wrong other than AuthFailNoAuthTokenCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store the ctx_result in the request extensions
    // It's a store by types
    req.extensions_mut().insert(result_ctx.clone());

    Ok(next.run(req).await)
}

// Parse a token of format `user-[user-id].[exp].[sign]`
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenInvalid)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

/**
 * Extractor
 * Extract AUTH_TOKEN cookie and parse it to get user_id
 * Don't need async_trait anymore
 */
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExtensions)?
            .clone()

        // TODO: Real auth-token parsing and validation
    }
}
