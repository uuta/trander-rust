use actix_web::error::ResponseError;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use actix_web::{get, App, HttpServer, Responder};
use std::fmt;

#[macro_use]
extern crate diesel;

mod db;
mod model;
mod schema;

#[derive(Debug)]
struct ApiError(String);

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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hella, World!")
}

#[get("/settings/{user_id}")]
async fn get(db: Data<db::DbPool>, path: Path<u64>) -> Result<impl Responder, ApiError> {
    use diesel::prelude::*;
    use schema::settings::dsl::*;

    let user_id_value = path.into_inner();
    let result = settings
        .filter(user_id.eq(user_id_value))
        .load::<model::Setting>(
            &mut db
                .get()
                .map_err(|err| ApiError(format!("Error getting connection: {}", err)))?,
        )
        .map_err(|err| ApiError(format!("Error loading settings: {}", err)))?;

    Ok(Json(result))
}

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
            .service(get)
            .service(index)
    })
    .bind((SERVER_IP, PORT))?
    .run()
    .await
}
