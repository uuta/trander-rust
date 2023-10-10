use crate::schema::request_limits;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize, Clone, PartialEq)]
pub struct RequestLimit {
    pub id: u32,
    pub user_id: u64,
    pub request_limit: u64,
    pub first_requested_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = request_limits)]
pub struct NewRequestLimit {
    pub user_id: u64,
    pub request_limit: u64,
    pub first_requested_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
