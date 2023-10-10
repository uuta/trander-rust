use crate::schema::google_place_ids;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize)]
pub struct GooglePlaceIds {
    pub id: u64,
    pub place_id: String,
    pub name: String,
    pub icon: String,
    pub rating: Option<f64>,
    pub photo: Option<String>,
    pub vicinity: Option<String>,
    pub user_ratings_total: Option<i32>,
    pub price_level: Option<i32>,
    pub lat: BigDecimal,
    pub lng: BigDecimal,
    pub rating_star: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = google_place_ids)]
pub struct NewGooglePlaceIds {
    pub place_id: String,
    pub name: String,
    pub icon: String,
    pub rating: Option<f64>,
    pub photo: Option<String>,
    pub vicinity: Option<String>,
    pub user_ratings_total: Option<i32>,
    pub price_level: Option<i32>,
    pub lat: BigDecimal,
    pub lng: BigDecimal,
    pub rating_star: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
