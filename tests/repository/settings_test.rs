use super::get_test_db_pool;
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use trander_rust::model::setting::NewSetting;
use trander_rust::model::user::NewUser;
use trander_rust::repository::settings;
use trander_rust::schema::{settings as settings_schema, users as users_schema};

#[actix_rt::test]
async fn test_get() {
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();

    match conn.transaction::<_, Error, _>(|conn| {
        let new_user = NewUser {
            id: 1,
            name: Some("test".to_string()),
            email: Some("aaa@test.com".to_string()),
            email_verified_at: Some(
                NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            password: Some("test".to_string()),
            created_at: Some(
                NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
        };

        let new_setting = NewSetting {
            user_id: 1,
            min_distance: 20,
            max_distance: 100,
            direction_type: 0,
            created_at: Some(
                NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
        };

        insert_into(users_schema::table)
            .values(&new_user)
            .execute(conn)
            .expect("Failed to insert new user");

        insert_into(settings_schema::table)
            .values(&new_setting)
            .execute(conn)
            .expect("Failed to insert new setting");

        let user_id_value = 1;
        let result = settings::get(user_id_value, conn);
        assert!(result.is_ok());

        let settings = result.unwrap();
        assert!(settings.len() > 0);

        for setting in settings {
            assert_eq!(setting.user_id, user_id_value);
        }

        Err::<(), diesel::result::Error>(diesel::result::Error::RollbackTransaction)
    }) {
        Err(Error::RollbackTransaction) => (), // do nothing for rollback
        Err(e) => panic!("Unexpected error: {}", e), // panic on other errors
        _ => (),
    }
}
