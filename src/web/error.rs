use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    AuthFailNoAuthTokenCookie,
    AuthFailCtxNotInRequestExtension,
    AuthFailTokenWrongFormat,
    TicketDeleteFailedIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--->> {:<12} - {self:?}", "INTO_RES");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Error::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Error::AuthFailNoAuthTokenCookie
            | Error::AuthFailCtxNotInRequestExtension
            | Error::AuthFailTokenWrongFormat => (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),
            // Model
            Error::TicketDeleteFailedIdNotFound { .. } => {
                (StatusCode::NOT_FOUND, ClientError::INVALID_PARAMETERS)
            }
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMETERS,
    SERVICE_ERROR,
    AUTH_FAIL,
    AUTH_FAIL_TOKEN_WRONG_FORMAT,
    AUTH_FAIL_CTX_NOT_IN_REQUEST_EXTENSION,
}
