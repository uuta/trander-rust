use actix_web::{get, HttpResponse, Responder};
use tracing::info;

#[get("/")]
pub async fn index() -> impl Responder {
    info!("Request to index");
    HttpResponse::Ok().body("Hello, World!")
}
