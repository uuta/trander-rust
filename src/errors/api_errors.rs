use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use std::fmt;

#[derive(Debug)]
pub struct ApiError(String);

impl ApiError {
    pub fn new(message: String) -> Self {
        ApiError(message)
    }
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(format!("An error occurred: {}", self.0))
    }
}
