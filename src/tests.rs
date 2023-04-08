use actix_http_test::{TestRequest, TestServer};
use actix_web::App;
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use serial_test::serial;
use std::env;

fn setup_app() -> App<impl actix_web::dev::ServiceFactory> {
    // Create the Actix-web app and configure it with your routes.
    // Replace `your_app_config` with the actual function to configure your app.
    your_app_config()
}

fn setup_db() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).unwrap()
}

async fn create_test_server() -> TestServer {
    TestServer::new(|| setup_app())
}
