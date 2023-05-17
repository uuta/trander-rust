use crate::model::setting::Setting;
use crate::schema;
use diesel::prelude::*;
use diesel::MysqlConnection;

use schema::settings::dsl::*;

pub fn get(
    user_id_value: u64,
    conn: &mut MysqlConnection,
) -> Result<Vec<Setting>, diesel::result::Error> {
    settings
        .filter(user_id.eq(user_id_value))
        .load::<Setting>(conn)
}
