use super::get_test_db_pool;
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use trander_rust::model::user::NewUser;
// Need to import SettingsRepository too
// https://chat.openai.com/c/52b673b5-ccde-4752-b90f-cf54914a9ca0
use trander_rust::repository::users::{ImplUsersRepository, UsersRepository};
use trander_rust::schema::users as users_schema;

#[cfg(test)]
mod tests {
    use super::*;
    // Import this to run tests in serial
    use serial_test::serial;

    #[actix_web::test]
    #[serial]
    async fn test_get_by_email() {
        let pool = get_test_db_pool().await;
        let mut conn = pool.get().unwrap();
        let repo = ImplUsersRepository;
        let email_value = "aaa@test.com".to_string();

        match conn.transaction::<_, Error, _>(|conn| {
            let new_user = NewUser {
                id: 1,
                name: Some("test".to_string()),
                email: Some(email_value.clone()),
                email_verified_at: Some(
                    NaiveDateTime::parse_from_str("2023-03-04 00:00:00", "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                ),
                password: Some("test".to_string()),
            };

            insert_into(users_schema::table)
                .values(&new_user)
                .execute(conn)
                .expect("Failed to insert new user");

            let user_id_value = 1;
            let result = repo.get_by_email(conn, &email_value);
            assert!(result.is_ok());
            assert_eq!(result.as_ref().unwrap().id, user_id_value);
            assert_eq!(result.as_ref().unwrap().email, Some(email_value.clone()));

            Err::<(), diesel::result::Error>(diesel::result::Error::RollbackTransaction)
        }) {
            Err(Error::RollbackTransaction) => (), // do nothing for rollback
            Err(e) => panic!("Unexpected error: {}", e), // panic on other errors
            _ => (),
        }
    }

    #[actix_web::test]
    #[serial]
    async fn test_add() {
        let pool = get_test_db_pool().await;
        let mut conn = pool.get().unwrap();
        let repo = ImplUsersRepository;
        let emails = &vec!["aaa@test.com".to_string(), "bbb@test.com".to_string()];

        match conn.transaction::<_, Error, _>(|conn| {
            for (i, email_value) in emails.iter().enumerate() {
                let _i_u64 = i as u64;
                let result = repo.add(conn, &email_value);
                assert!(result.is_ok());
                // INFO: fix the architecture of the whole test in repository
                // the repository returns incremented id after rollback transaction
                // assert_eq!(result.unwrap(), i_u64 + 1);
            }

            Err::<(), diesel::result::Error>(diesel::result::Error::RollbackTransaction)
        }) {
            Err(Error::RollbackTransaction) => (), // do nothing for rollback
            Err(e) => panic!("Unexpected error: {}", e), // panic on other errors
            _ => (),
        }
    }
}
