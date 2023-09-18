use crate::error_log;
use actix_web::error::ResponseError;
use actix_web::{http::StatusCode, HttpResponse};
use r2d2;
use serde::Serialize;
use std::fmt;
use tracing::error;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum HttpErrorType {
    DbError,
    DbConnectionError,
    ValidationError,
    NotFoundError,
    NetworkError,
    HeaderParseError,
    JsonParseError,
    IOError,
}

// Struct type is already defined Option<String> and HttpErrorType. We can also define later.
#[derive(Debug)]
pub struct HttpError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: HttpErrorType,
}

#[derive(Serialize)]
pub struct HttpErrorResponse {
    pub error: String,
}

/// @see https://dev.to/chaudharypraveen98/error-handling-in-actix-web-4mm
/// @see https://github.com/chaudharypraveen98/actix-question-bank-stackoverflow/blob/master/src/error.rs
impl HttpError {
    // we are handling the none. function name should match field name
    fn message(&self) -> String {
        match &*self {
            // Error message is found then clone otherwise default message
            HttpError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            HttpError {
                cause: Some(cause),
                message: None,
                error_type: HttpErrorType::DbError,
            } => cause.clone(),
            HttpError {
                cause: Some(cause),
                message: None,
                error_type: HttpErrorType::DbConnectionError,
            } => cause.clone(),
            HttpError {
                cause: _,
                message: None,
                error_type: HttpErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            HttpError {
                cause: Some(cause),
                message: None,
                error_type: HttpErrorType::ValidationError,
            } => cause.clone(),
            HttpError {
                cause: Some(cause),
                message: None,
                error_type: HttpErrorType::NetworkError,
            } => cause.clone(),
            HttpError {
                cause: Some(cause),
                message: None,
                error_type: HttpErrorType::HeaderParseError,
            } => cause.clone(),
            HttpError {
                cause: Some(cause),
                message: None,
                error_type: HttpErrorType::JsonParseError,
            } => cause.clone(),
            HttpError {
                cause: Some(cause),
                message: None,
                error_type: HttpErrorType::IOError,
            } => cause.clone(),
            _ => "An unexpected error has occured".to_string(),
        }
    }
    // This db_error is used when we haven't implmented the From trait

    // pub fn db_error(error: impl ToString) -> HttpError {
    //     HttpError {
    //         cause: Some(error.to_string()),
    //         message: None,
    //         error_type: HttpErrorType::DbError,
    //     }
    // }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for HttpError {
    //error_response and status_code are the provided methods for ResponseError Trait

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            HttpErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpErrorType::DbConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpErrorType::NotFoundError => StatusCode::NOT_FOUND,
            HttpErrorType::ValidationError => StatusCode::LENGTH_REQUIRED,
            HttpErrorType::NetworkError => StatusCode::BAD_REQUEST,
            HttpErrorType::HeaderParseError => StatusCode::BAD_REQUEST,
            HttpErrorType::JsonParseError => StatusCode::BAD_REQUEST,
            HttpErrorType::IOError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        error_log!(self);
        HttpResponse::build(self.status_code()).json(HttpErrorResponse {
            error: self.message(),
        })
    }
}

impl From<diesel::result::Error> for HttpError {
    fn from(error: diesel::result::Error) -> HttpError {
        HttpError {
            message: None,
            cause: Some(error.to_string()),
            error_type: HttpErrorType::DbError,
        }
    }
}

impl From<r2d2::Error> for HttpError {
    fn from(error: r2d2::Error) -> HttpError {
        HttpError {
            message: None,
            cause: Some(error.to_string()),
            error_type: HttpErrorType::DbConnectionError,
        }
    }
}

impl From<ValidationErrors> for HttpError {
    fn from(error: ValidationErrors) -> HttpError {
        HttpError {
            message: None,
            cause: Some(error.to_string()),
            error_type: HttpErrorType::ValidationError,
        }
    }
}

impl From<reqwest::Error> for HttpError {
    fn from(error: reqwest::Error) -> HttpError {
        HttpError {
            message: None,
            cause: Some(error.to_string()),
            error_type: HttpErrorType::NetworkError,
        }
    }
}

impl From<diesel::result::Error> for HttpErrorType {
    fn from(_error: diesel::result::Error) -> HttpErrorType {
        HttpErrorType::DbError
    }
}

impl From<reqwest::header::InvalidHeaderValue> for HttpError {
    fn from(error: reqwest::header::InvalidHeaderValue) -> HttpError {
        HttpError {
            message: None,
            cause: Some(error.to_string()),
            error_type: HttpErrorType::HeaderParseError,
        }
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(error: serde_json::Error) -> HttpError {
        HttpError {
            message: None,
            cause: Some(error.to_string()),
            error_type: HttpErrorType::JsonParseError,
        }
    }
}

impl From<std::io::Error> for HttpError {
    fn from(error: std::io::Error) -> HttpError {
        HttpError {
            message: None,
            cause: Some(error.to_string()),
            error_type: HttpErrorType::IOError,
        }
    }
}

impl fmt::Display for HttpErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl ResponseError for HttpErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

#[cfg(test)]
mod tests {

    use super::{HttpError, HttpErrorType};
    use actix_web::error::ResponseError;

    #[test]
    fn test_default_db_error() {
        let db_error = HttpError {
            message: None,
            cause: None,
            error_type: HttpErrorType::DbError,
        };

        assert_eq!(
            db_error.message(),
            "An unexpected error has occured".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_default_not_found_error() {
        let db_error = HttpError {
            message: None,
            cause: None,
            error_type: HttpErrorType::NotFoundError,
        };

        assert_eq!(
            db_error.message(),
            "The requested item was not found".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_user_db_error() {
        let user_message = "User-facing message".to_string();

        let db_error = HttpError {
            message: Some(user_message.clone()),
            cause: None,
            error_type: HttpErrorType::DbError,
        };

        assert_eq!(
            db_error.message(),
            user_message,
            "User-facing message should be shown"
        );
    }

    #[test]
    fn test_db_error_status_code() {
        let expected = 500;

        let db_error = HttpError {
            message: None,
            cause: None,
            error_type: HttpErrorType::DbError,
        };

        assert_eq!(
            db_error.status_code(),
            expected,
            "Status code for DbError should be {}",
            expected
        );
    }
}
