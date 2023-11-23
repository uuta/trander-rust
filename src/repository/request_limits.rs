use crate::model::request_limits::{NewRequestLimit, RequestLimit};
use crate::schema;
use diesel::prelude::*;
use diesel::MysqlConnection;
use mockall::automock;
use schema::request_limits::dsl::*;

#[automock]
pub trait RequestLimitsRepository {
    fn get(
        &self,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<RequestLimit>, diesel::result::Error>;

    fn decrement(
        &self,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<(), diesel::result::Error>;

    fn add(
        &self,
        conn: &mut MysqlConnection,
        user_id_value: &u64,
    ) -> Result<usize, diesel::result::Error>;
}

pub struct ImplRequestLimitsRepository;

impl RequestLimitsRepository for ImplRequestLimitsRepository {
    fn get(
        &self,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<RequestLimit>, diesel::result::Error> {
        request_limits
            .filter(user_id.eq(user_id_value))
            .load::<RequestLimit>(conn)
    }

    fn decrement(
        &self,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(
            request_limits
                .filter(user_id.eq(user_id_value))
                .filter(request_limit.gt(0)),
        )
        .set(request_limit.eq(request_limit - 1))
        .execute(conn)
        .map(|_| ())
    }

    fn add(
        &self,
        conn: &mut MysqlConnection,
        user_id_value: &u64,
    ) -> Result<usize, diesel::result::Error> {
        let insert = NewRequestLimit {
            user_id: *user_id_value,
            request_limit: 10,
            first_requested_at: chrono::Utc::now().naive_utc(),
        };
        diesel::insert_into(request_limits)
            .values(&insert)
            .execute(conn)
    }
}
