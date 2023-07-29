use crate::model::google_place_ids::GooglePlaceIds;
use crate::schema;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Nullable, Text};
use diesel::MysqlConnection;
use mockall::automock;
use schema::google_place_ids::dsl::*;

#[automock]
pub trait GooglePlaceIdsRepository {
    fn get(
        &self,
        place_id_value: String,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<GooglePlaceIds>, diesel::result::Error>;

    fn upsert(
        &self,
        conn: &mut MysqlConnection,
        params: UpsertParams,
    ) -> Result<usize, diesel::result::Error>;
}

pub struct ImplGooglePlaceIdsRepository;

pub struct UpsertParams {
    pub place_id: String,
    pub name: String,
    pub icon: String,
    pub rating: Option<f64>,
    pub photo: Option<String>,
    pub vicinity: Option<String>,
    pub user_ratings_total: Option<i64>,
    pub price_level: Option<i64>,
    pub lat: f64,
    pub lng: f64,
    pub rating_star: Option<i64>,
}

impl GooglePlaceIdsRepository for ImplGooglePlaceIdsRepository {
    fn get(
        &self,
        place_id_value: String,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<GooglePlaceIds>, diesel::result::Error> {
        google_place_ids
            .filter(place_id.eq(place_id_value))
            .load::<GooglePlaceIds>(conn)
    }

    fn upsert(
        &self,
        conn: &mut MysqlConnection,
        params: UpsertParams,
    ) -> Result<usize, diesel::result::Error> {
        let sql = diesel::sql_query(
            "
        INSERT INTO google_place_ids (
            place_id, name, icon, rating, photo, vicinity,
            user_ratings_total, price_level, lat, lng, rating_star, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW())
        ON DUPLICATE KEY UPDATE
            name = VALUES(name),
            icon = VALUES(icon),
            rating = VALUES(rating),
            photo = VALUES(photo),
            vicinity = VALUES(vicinity),
            user_ratings_total = VALUES(user_ratings_total),
            price_level = VALUES(price_level),
            lat = VALUES(lat),
            lng = VALUES(lng),
            rating_star = VALUES(rating_star),
            updated_at = NOW()
    ",
        )
        .bind::<Text, _>(params.place_id)
        .bind::<Text, _>(params.name)
        .bind::<Text, _>(params.icon)
        .bind::<Nullable<Double>, _>(params.rating)
        .bind::<Nullable<Text>, _>(params.photo)
        .bind::<Nullable<Text>, _>(params.vicinity)
        .bind::<Nullable<BigInt>, _>(params.user_ratings_total)
        .bind::<Nullable<BigInt>, _>(params.price_level)
        .bind::<Double, _>(params.lat)
        .bind::<Double, _>(params.lng)
        .bind::<Nullable<BigInt>, _>(params.rating_star);

        sql.execute(conn)
    }
}
