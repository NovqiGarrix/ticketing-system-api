use actix_web::{
    HttpResponse, HttpResponseBuilder,
    body::BoxBody,
    error::ResponseError,
    http::{StatusCode, header},
    mime,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation")]
    Validation(Vec<FieldError>),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("An unexpected internal error occured")]
    InternalServerError {
        #[from]
        source: anyhow::Error,
    },

    #[error("A Database error occured")]
    Database(
        #[from]
        #[source]
        sea_orm::DbErr,
    ),
}

impl AppError {
    pub fn log_error(&self) {
        match self {
            Self::InternalServerError { source } => {
                log::error!("InternalServerError: {:?}", source)
            }
            Self::Database(err) => log::error!("Database Error: {:?}", err),

            _ => log::info!("Clieng Error: {}", self),
        }
    }

    pub fn get_error_message(&self) -> String {
        match self {
            Self::Validation(_) => "ValidationError".to_string(),
            Self::InternalServerError { .. } | Self::Database(_) => {
                "An internal server error occured. Please try again later.".to_string()
            }
            _ => self.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse<'a> {
    pub code: u16,
    pub message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<&'a [FieldError]>,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Validation(_) | Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,

            // All other errors are server-side issues.
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        // Log error
        self.log_error();

        let status_code = self.status_code();

        let mut builder = HttpResponseBuilder::new(status_code.clone());
        builder.insert_header(header::ContentType(mime::APPLICATION_JSON));

        let message = self.get_error_message();

        match self {
            Self::Validation(errors) => builder.json(AppErrorResponse {
                code: status_code.as_u16(),
                message: &message,
                validation_errors: Some(errors),
            }),

            _ => builder.json(AppErrorResponse {
                code: status_code.as_u16(),
                message: &message,
                validation_errors: None,
            }),
        }
    }
}
