use crate::info_request_log;
use actix_web::{get, HttpResponse, Responder};
use tracing::info;

#[get("/")]
pub async fn index() -> impl Responder {
    info_request_log!();
    HttpResponse::Ok().body("Hello, World!")
}
