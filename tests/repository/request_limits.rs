use super::get_test_db_pool;
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use trander_rust::model::request_limits::NewRequestLimit;
use trander_rust::model::user::NewUser;
use trander_rust::repository::request_limits::{
    ImplRequestLimitsRepository, RequestLimitsRepository,
};
use trander_rust::schema::{request_limits as request_limits_schema, users as users_schema};

#[actix_rt::test]
async fn test_get() {
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();
    let repo = ImplRequestLimitsRepository;

    match conn.transaction::<_, Error, _>(|conn| {
        let new_user = NewUser {
            id: 1,
            name: Some("test".to_string()),
            email: Some("aaa@test.com".to_string()),
            email_verified_at: Some(
                NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            password: Some("test".to_string()),
            created_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        };

        let new_request_limit = NewRequestLimit {
            user_id: 1,
            request_limit: 100,
            first_requested_at: NaiveDateTime::parse_from_str(
                "2023-03-04 00:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            created_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        };

        insert_into(users_schema::table)
            .values(&new_user)
            .execute(conn)
            .expect("Failed to insert new user");

        insert_into(request_limits_schema::table)
            .values(&new_request_limit)
            .execute(conn)
            .expect("Failed to insert new request limit");

        let user_id_value = 1;
        let result = repo.get(user_id_value, conn);
        assert!(result.is_ok());

        let got = result.unwrap();
        assert!(got.len() > 0);

        for r in got {
            assert_eq!(r.request_limit, 100);
        }

        Err::<(), diesel::result::Error>(diesel::result::Error::RollbackTransaction)
    }) {
        Err(Error::RollbackTransaction) => (), // do nothing for rollback
        Err(e) => panic!("Unexpected error: {}", e), // panic on other errors
        _ => (),
    }
}

#[actix_rt::test]
async fn test_decrement() {
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();
    let repo = ImplRequestLimitsRepository;

    match conn.transaction::<_, Error, _>(|conn| {
        let new_user = NewUser {
            id: 1,
            name: Some("test".to_string()),
            email: Some("aaa@test.com".to_string()),
            email_verified_at: Some(
                NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            password: Some("test".to_string()),
            created_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        };

        let new_request_limit = NewRequestLimit {
            user_id: 1,
            request_limit: 100,
            first_requested_at: NaiveDateTime::parse_from_str(
                "2023-03-04 00:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            created_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        };

        insert_into(users_schema::table)
            .values(&new_user)
            .execute(conn)
            .expect("Failed to insert new user");

        insert_into(request_limits_schema::table)
            .values(&new_request_limit)
            .execute(conn)
            .expect("Failed to insert new request limit");

        let user_id_value = 1;
        repo.decrement(user_id_value, conn).unwrap();
        let result = repo.get(user_id_value, conn);
        assert!(result.is_ok());

        let got = result.unwrap();
        assert!(got.len() > 0);

        for r in got {
            assert_eq!(r.request_limit, 99);
        }

        Err::<(), diesel::result::Error>(diesel::result::Error::RollbackTransaction)
    }) {
        Err(Error::RollbackTransaction) => (), // do nothing for rollback
        Err(e) => panic!("Unexpected error: {}", e), // panic on other errors
        _ => (),
    }
}

#[actix_rt::test]
async fn test_decrement_if_zero() {
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();
    let repo = ImplRequestLimitsRepository;

    match conn.transaction::<_, Error, _>(|conn| {
        let new_user = NewUser {
            id: 1,
            name: Some("test".to_string()),
            email: Some("aaa@test.com".to_string()),
            email_verified_at: Some(
                NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            password: Some("test".to_string()),
            created_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        };

        let new_request_limit = NewRequestLimit {
            user_id: 1,
            request_limit: 0,
            first_requested_at: NaiveDateTime::parse_from_str(
                "2023-03-04 00:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            created_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        };

        insert_into(users_schema::table)
            .values(&new_user)
            .execute(conn)
            .expect("Failed to insert new user");

        insert_into(request_limits_schema::table)
            .values(&new_request_limit)
            .execute(conn)
            .expect("Failed to insert new request limit");

        let user_id_value = 1;
        repo.decrement(user_id_value, conn).unwrap();
        let result = repo.get(user_id_value, conn);
        assert!(result.is_ok());

        let got = result.unwrap();
        assert!(got.len() > 0);

        for r in got {
            assert_eq!(r.request_limit, 0);
        }

        Err::<(), diesel::result::Error>(diesel::result::Error::RollbackTransaction)
    }) {
        Err(Error::RollbackTransaction) => (),
        Err(e) => panic!("Unexpected error: {}", e),
        _ => (),
    }
}
