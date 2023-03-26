use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    match Pool::builder().build(manager) {
        Ok(pool) => {
            println!("Successfully connected to the database.");
            pool
        }
        Err(e) => {
            eprintln!("Failed to create pool: {}", e);
            panic!("Failed to create pool.");
        }
    }
}
