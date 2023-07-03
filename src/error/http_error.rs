use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use std::fmt;

#[derive(Debug)]
pub struct HttpError {
    name: &'static str,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match self.name {
            "BadRequest" => HttpResponse::BadRequest().json("An error occurred"),
            "Unauthorized" => HttpResponse::Unauthorized().json("Unauthorized"),
            "NotFound" => HttpResponse::NotFound().json("Not found"),
            "OtherClientError" => HttpResponse::BadRequest().json("An error occurred"),
            "InternalServerError" => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            _ => HttpResponse::InternalServerError().json("Unknown error"),
        }
    }
}

impl std::error::Error for HttpError {}
