use super::get_test_db_pool;
use trander_rust::repository::settings::MockSettingsRepository;
use trander_rust::service::settings::get;

#[actix_rt::test]
async fn test_get() {
    let mut mock_repo = MockSettingsRepository::new();
    let pool = get_test_db_pool().await;
    let mut conn = pool.get().unwrap();

    mock_repo.expect_get().returning(|_, _| Ok(vec![])); // Return whatever you want here

    let result = get(&mock_repo, 1, &mut conn);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}
