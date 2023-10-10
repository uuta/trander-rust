use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize)]
pub struct User {
    pub id: u64,
    pub unique_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: Option<String>,
    pub remember_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: u64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: Option<String>,
}
