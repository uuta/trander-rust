use crate::model::google_place_ids::GooglePlaceIds;
use crate::schema;
use diesel::prelude::*;
use diesel::MysqlConnection;

use schema::google_place_ids::dsl::*;

pub fn get(
    place_id_value: String,
    conn: &mut MysqlConnection,
) -> Result<Vec<GooglePlaceIds>, diesel::result::Error> {
    google_place_ids
        .filter(place_id.eq(place_id_value))
        .load::<GooglePlaceIds>(conn)
}
