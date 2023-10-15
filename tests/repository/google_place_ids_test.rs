use super::get_test_db_pool;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use std::str::FromStr;
use trander_rust::model::google_place_ids::NewGooglePlaceIds;
use trander_rust::repository::google_place_ids::{
    GooglePlaceIdsRepository, ImplGooglePlaceIdsRepository, UpsertParams,
};
use trander_rust::schema::google_place_ids as google_place_ids_schema;

#[cfg(test)]
mod tests {
    use super::*;
    // Import this to run tests in serial
    use serial_test::serial;

    #[actix_web::test]
    #[serial]
    async fn test_upsert() {
        let pool = get_test_db_pool().await;
        let mut conn = pool.get().unwrap();
        let repo = ImplGooglePlaceIdsRepository;

        let google_place_id_row = NewGooglePlaceIds {
            place_id: "test".to_string(),
            name: "test".to_string(),
            icon: "test".to_string(),
            rating: Some(4.0),
            photo: None,
            vicinity: None,
            user_ratings_total: Some(22),
            price_level: None,
            lat: BigDecimal::from_str("12.0").unwrap(),
            lng: BigDecimal::from_str("13.0").unwrap(),
            rating_star: None,
            created_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            updated_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        };

        match conn.transaction::<_, Error, _>(|conn| {
            insert_into(google_place_ids_schema::table)
                .values(&google_place_id_row)
                .execute(conn)
                .expect("Failed to insert new user");

            let params = UpsertParams {
                place_id: "test2".to_string(),
                name: "test2".to_string(),
                icon: "test2".to_string(),
                rating: Some(5.0),
                photo: None,
                vicinity: None,
                user_ratings_total: Some(22),
                price_level: None,
                lat: 12.0,
                lng: 13.0,
                rating_star: None,
            };
            let result = repo.upsert(conn, params);
            assert!(result.is_ok());

            let upserted_count = result.unwrap();
            assert!(upserted_count == 1);

            Err::<(), diesel::result::Error>(diesel::result::Error::RollbackTransaction)
        }) {
            Err(Error::RollbackTransaction) => (), // do nothing for rollback
            Err(e) => panic!("Unexpected error: {}", e), // panic on other errors
            _ => (),
        }
    }
}
