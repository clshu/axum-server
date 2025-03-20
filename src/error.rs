use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
// use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // Auth Error
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailTokenExpired,
    AuthFailTokenInvalid,

    // -- Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
}

// Don't expose server errors to the client.
// Pick and choose which errors to expose later.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLINET_ERROR").into_response()
    }
}

// region:    --- Custom

// impl Error {
//     pub fn custom(val: impl std::fmt::Display) -> Self {
//         Self::Custom(val.to_string())
//     }
// }

// impl From<&str> for Error {
//     fn from(val: &str) -> Self {
//         Self::Custom(val.to_string())
//     }
// }

// endregion: --- Custom

// region:    --- Error Boilerplate

// impl std::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
//         write!(fmt, "{self:?}")
//     }
// }

// impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
