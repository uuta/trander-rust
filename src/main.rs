use actix_web::web::Data;
use actix_web::{App, HttpServer};
use tracing::Level;
use tracing_subscriber;
use trander_rust::db;
use trander_rust::handler;
use trander_rust::middleware;

extern crate diesel;

// Using 127.0.0.1 or localhost here won’t work from inside docker.
// Ref: https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
const SERVER_IP: &str = "0.0.0.0";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Output DEBUG level logs and above
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // import establish_connection method from db module
    let pool = db::establish_connection();

    // Inject db pool to actix_web by app_data
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::jwt::JWTProcessing)
            .wrap(middleware::post_processing::PostProcessing)
            .service(handler::settings::get)
            .service(handler::index::index)
            .service(handler::cities::get)
            .service(handler::near_by_search::get)
            .service(handler::backpacker::get)
    })
    .bind((SERVER_IP, PORT))?
    .run()
    .await
}
