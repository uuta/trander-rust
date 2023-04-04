use crate::model;
use crate::repository::settings;
use diesel::MysqlConnection;

pub fn get(
    user_id_value: u64,
    conn: &mut MysqlConnection,
) -> Result<Vec<model::Setting>, diesel::result::Error> {
    settings::get(user_id_value, conn)
}
