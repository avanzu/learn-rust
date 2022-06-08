use actix_web::error::{BlockingError, ResponseError};
use actix_web::HttpResponse;
use derive_more::Display;
use diesel::result::Error as DBError;
use r2d2::Error as PoolingError;
use DBError::NotFound;

#[derive(Debug, Display)]
pub enum ServiceError {
    GeneralError(String),
    NotFound(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            ServiceError::GeneralError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }
}

impl From<BlockingError> for ServiceError {
    fn from(err: BlockingError) -> Self {
        ServiceError::GeneralError(format!("BlockingError: {}", err))
    }
}

impl From<DBError> for ServiceError {
    fn from(err: DBError) -> Self {
        match err {
            NotFound => ServiceError::NotFound(format!("{}", err)),
            _ => ServiceError::GeneralError(format!("DBError: {}", err)),
        }
    }
}

impl From<PoolingError> for ServiceError {
    fn from(err: PoolingError) -> Self {
        ServiceError::GeneralError(format!("PoolingError: {}", err))
    }
}
impl From<argon2::Error> for ServiceError {
    fn from(err: argon2::Error) -> Self {
        ServiceError::GeneralError(format!("PasswordHashError: {}", err))
    }
}
