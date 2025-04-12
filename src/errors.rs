use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Database error")]
    DbError,
    #[display(fmt = "Asset not found")]
    NotFound,
    #[display(fmt = "Bad request: {}", _0)]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DbError => HttpResponse::InternalServerError().json("Internal Server Error"),
            AppError::NotFound => HttpResponse::NotFound().json("Not Found"),
            AppError::BadRequest(message) => HttpResponse::BadRequest().json(message),
        }
    }
}
impl std::convert::From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        AppError::DbError
    }
}