use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

async fn get_test_db_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create test database connection pool.")
}

#[cfg(test)]
mod repository;
