use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    HttpServer::new(move || App::new().route("/index", web::get().to(manual_hello)))
        // Using 127.0.0.1 or localhost here won’t work from inside docker.
        // Ref: https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
