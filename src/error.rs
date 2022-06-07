use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use askama::Template;
use serde::Serialize;
use thiserror::Error;

use crate::message::{Index, Message, MessageKind};

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
    #[error("Link has invalid hostname")]
    InvalidLink,
    #[error("Link has invalid scheme")]
    InvalidScheme,
    #[error("Database error")]
    DatabaseError(#[from] rocksdb::Error),
    #[error("Unable to decode short link")]
    HasherError(#[from] harsh::Error),
    #[error("Templating error")]
    TemplateError(#[from] askama::shared::Error),
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
            Self::InvalidScheme => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HasherError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let index = Index::new(Some(&MessageKind::Error(Message {
            title: &self.status_code().as_str(),
            body: &self.to_string(),
        })))
        .render();

        match index {
            Ok(index) => HttpResponse::Ok().content_type("text/html").body(index),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
