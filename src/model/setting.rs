use crate::schema::settings;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize, Clone, PartialEq)]
pub struct Setting {
    pub id: u64,
    pub user_id: u64,
    pub min_distance: i32,
    pub max_distance: i32,
    pub direction_type: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub user_id: u64,
    pub min_distance: i32,
    pub max_distance: i32,
    pub direction_type: i16,
    pub created_at: NaiveDateTime,
}
