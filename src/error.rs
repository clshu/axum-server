use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
// use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    LoginFail,

    // Auth Error
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailTokenExpired,
    AuthFailTokenInvalid,
    AuthFailCtxNotInRequestExtensions,

    // -- Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}

// Don't expose server errors to the client.
// Pick and choose which errors to expose later.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // A placeholder response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert Error into response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn cinet_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            //    Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenExpired
            | Self::AuthFailTokenInvalid
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExtensions => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            //    Model
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
