use crate::model::setting::Setting;
use crate::schema;
use diesel::prelude::*;
use diesel::MysqlConnection;
use mockall::automock;

#[automock]
pub trait SettingsRepository {
    fn get(
        &self,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<Setting>, diesel::result::Error>;
}

pub struct RealSettingsRepository;

use schema::settings::dsl::*;

impl SettingsRepository for RealSettingsRepository {
    fn get(
        &self,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<Setting>, diesel::result::Error> {
        settings
            .filter(user_id.eq(user_id_value))
            .load::<Setting>(conn)
    }
}
