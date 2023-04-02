use actix_web::web::Data;
use actix_web::{App, HttpServer};

#[macro_use]
extern crate diesel;

mod db;
mod errors;
mod handlers;
mod model;
mod schema;

// INFO:
// Using 127.0.0.1 or localhost here won’t work from inside docker.
// Ref: https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
const SERVER_IP: &str = "0.0.0.0";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // db moduleからestablish_connection関数をimport
    let pool = db::establish_connection();

    // app_dataを用いactix_webにdb poolをinject
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(handlers::settings::get)
            .service(handlers::index::index)
    })
    .bind((SERVER_IP, PORT))?
    .run()
    .await
}
