use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

// Reference:
// https://mattgathu.github.io/2020/04/16/actix-web-error-handling.html

#[derive(Error, Debug)]
pub enum Error {
    #[error("Requested file was not found")]
    NotFound,
    #[error("Unknown internal error")]
    Unknown,
    #[error("Parse error")]
    ParseError(#[from] url::ParseError),
    #[error("Failed to generate resource URI")]
    UrlGenerationError(#[from] actix_web::error::UrlGenerationError),
    #[error("Link either has invalid scheme or hostname")]
    InvalidLink,
    #[error("Database error")]
    DatabaseError(#[from] rocksdb::Error),
    #[error("Hasher error")]
    HasherError(#[from] harsh::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ParseError(_) => StatusCode::BAD_REQUEST,
            Self::UrlGenerationError(_) => StatusCode::BAD_REQUEST,
            Self::InvalidLink => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HasherError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            code: self.status_code().as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
}
