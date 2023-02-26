use actix_web::web::Data;
use actix_web::{get, web, App, HttpServer, Responder};

#[macro_use]
extern crate diesel;

mod db;
mod model;
mod schema;

#[get("/users/{id}")]
async fn get(db: web::Data<db::DbPool>, id: web::Path<String>) -> impl Responder {
    // db connectionが利用可能に！
    let _conn = db.get().unwrap();
    format!("Hello {id}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // db moduleからestablish_connection関数をimport
    let pool = db::establish_connection();

    // app_dataを用いactix_webにdb poolをinject
    HttpServer::new(move || App::new().app_data(Data::new(pool.clone())).service(get))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
