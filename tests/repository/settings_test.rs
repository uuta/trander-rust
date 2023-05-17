use super::get_test_db_pool;
use trander_rust::schema::settings as settings_schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::insert_into;
use trander_rust::repository::settings;
use trander_rust::model::NewSetting;

#[actix_rt::test]
async fn test_get() {
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();

    let new_setting = NewSetting {
        user_id: 1,
        min_distance: 20,
        max_distance: 100,
        direction_type: 0,
        created_at: Some(NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
    };

    insert_into(settings_schema::table)
        .values(&new_setting)
        .execute(&mut conn)
        .expect("Failed to insert new setting");

    let user_id_value = 1;
    let result = settings::get(user_id_value, &mut conn);
    assert!(result.is_ok());

    let settings = result.unwrap();
    assert!(settings.len() > 0);

    for setting in settings {
        assert_eq!(setting.user_id, user_id_value);
    }
}
