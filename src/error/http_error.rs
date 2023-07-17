use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use std::fmt;

#[derive(Debug)]
pub struct HttpError {
    code: &'static str,
    message: String,
}

impl HttpError {
    pub fn new(code: &'static str, message: String) -> Self {
        HttpError { code, message }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self.code {
            "BadRequest" => HttpResponse::BadRequest().json(&self.message),
            "Unauthorized" => HttpResponse::Unauthorized().json(&self.message),
            "NotFound" => HttpResponse::NotFound().json(&self.message),
            "ApiRequestError" => HttpResponse::InternalServerError().json(&self.message),
            "OtherClientError" => HttpResponse::BadRequest().json(&self.message),
            "InternalServerError" => HttpResponse::InternalServerError().json(&self.message),
            _ => HttpResponse::InternalServerError().json(&self.message),
        }
    }
}

impl std::error::Error for HttpError {}
