use crate::model::user::User;
use crate::schema;
use diesel::prelude::*;
use diesel::MysqlConnection;
use mockall::automock;
use schema::users::dsl::*;

#[automock]
pub trait UsersRepository {
    fn get_by_email(
        &self,
        conn: &mut MysqlConnection,
        email_value: String,
    ) -> Result<Vec<User>, diesel::result::Error>;

    fn add(
        &self,
        conn: &mut MysqlConnection,
        email_value: String,
    ) -> Result<usize, diesel::result::Error>;
}

pub struct ImplUsersRepository;

impl UsersRepository for ImplUsersRepository {
    fn get_by_email(
        &self,
        conn: &mut MysqlConnection,
        email_value: String,
    ) -> Result<Vec<User>, diesel::result::Error> {
        users.filter(email.eq(email_value)).load::<User>(conn)
    }

    fn add(
        &self,
        conn: &mut MysqlConnection,
        email_value: String,
    ) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(users)
            .values(email.eq(email_value))
            .execute(conn)
    }
}
