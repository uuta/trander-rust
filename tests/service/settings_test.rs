use super::get_test_db_pool;
use chrono::NaiveDateTime;
use trander_rust::repository::settings::MockSettingsRepository;
use trander_rust::service::settings::get;

#[actix_rt::test]
async fn test_get_ok() {
    let mut mock_repo = MockSettingsRepository::new();
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();

    let settings = vec![trander_rust::model::setting::Setting {
        id: 1,
        user_id: 1,
        min_distance: 0,
        max_distance: 100,
        direction_type: 0,
        created_at: Some(
            NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        updated_at: NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        deleted_at: Some(
            NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
    }];
    let settings_clone = settings.clone();
    mock_repo.expect_get().return_once(move |_, _| Ok(settings));

    let result = get(&mock_repo, 1, &mut conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), settings_clone);
}

#[actix_rt::test]
async fn test_get_ok_empty() {
    let mut mock_repo = MockSettingsRepository::new();
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();

    mock_repo.expect_get().returning(|_, _| Ok(vec![]));

    let result = get(&mock_repo, 1, &mut conn);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}
