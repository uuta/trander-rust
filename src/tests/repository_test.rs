mod utils;

#[cfg(test)]
mod tests {
    use crate::model::Setting;
    use crate::repository::settings;
    use utils::get_test_db_pool;

    #[actix_rt::test]
    async fn test_get() {
        let pool = get_test_db_pool();
        let mut conn = pool.get().unwrap();
        let user_id_value = 1;

        let result = settings::get(user_id_value, &mut conn);
        assert!(result.is_ok());

        let settings = result.unwrap();
        assert!(settings.len() > 0);

        for setting in settings {
            assert_eq!(setting.user_id, user_id_value);
        }
    }
}
