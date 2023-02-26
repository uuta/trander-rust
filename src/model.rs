use crate::schema::settings;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "settings"]
pub struct Setting {
    pub id: Option<u64>,
    pub user_id: u64,
    pub min_distance: i32,
    pub max_distance: i32,
    pub direction_type: i16,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
