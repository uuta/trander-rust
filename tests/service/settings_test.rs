use super::get_test_db_pool;
use chrono::NaiveDateTime;
use trander_rust::repository::settings::{MockSettingsRepository, UpdateParams};
use trander_rust::service::settings::{ImplSettingsService, SettingsService};

#[actix_rt::test]
async fn test_get_ok() {
    let mut mock_repo = MockSettingsRepository::new();
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();
    let service = ImplSettingsService;

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

    let result = service.get(&mock_repo, 1, &mut conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), settings_clone);
}

#[actix_rt::test]
async fn test_get_ok_empty() {
    let mut mock_repo = MockSettingsRepository::new();
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();
    let service = ImplSettingsService;

    mock_repo.expect_get().returning(|_, _| Ok(vec![]));

    let result = service.get(&mock_repo, 1, &mut conn);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[actix_rt::test]
async fn test_update_ok() {
    let mut mock_repo = MockSettingsRepository::new();
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();
    let service = ImplSettingsService;
    let expected_user_id = 1;

    let expected_params = UpdateParams {
        min_distance: 0,
        max_distance: 100,
        direction_type: 0,
    };

    mock_repo
        .expect_update()
        .return_once(move |_, user_id, params| {
            assert_eq!(user_id, expected_user_id);
            assert_eq!(params, expected_params.clone());
            Ok(1)
        });

    let result = service.update(&mock_repo, &mut conn, expected_user_id, expected_params);
    assert!(result.is_ok());
}

#[actix_rt::test]
async fn test_update_err() {
    let mut mock_repo = MockSettingsRepository::new();
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();
    let service = ImplSettingsService;
    let expected_user_id = 24;

    let expected_params = UpdateParams {
        min_distance: 0,
        max_distance: 100,
        direction_type: 0,
    };

    mock_repo
        .expect_update()
        .return_once(move |_, user_id, params| {
            assert_eq!(user_id, expected_user_id);
            assert_eq!(params, expected_params.clone());
            Err(diesel::result::Error::NotFound)
        });

    let result = service.update(&mock_repo, &mut conn, expected_user_id, expected_params);
    assert!(result.is_err());
}
