use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use lazy_regex::regex_captures;

use tower_cookies::Cookies;

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: u64,
}

// Constructor
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Getters
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
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

        // Use cookies extractor
        let cookies = Cookies::from_request_parts(parts, state).await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // TODO: Real auth-token parsing and validation

        Ok(Ctx::new(user_id))
    }
}

// Parse a token of format `user-[user-id].[exp].[sign]`
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenInvalid)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
