use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::api::responses::Error::NotFound;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound,
    ServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            NotFound => (StatusCode::NOT_FOUND, StatusCode::NOT_FOUND.to_string()).into_response(),
            Error::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, StatusCode::INTERNAL_SERVER_ERROR.to_string()).into_response()
        }
    }
}
