use crate::model::setting::Setting;
use crate::schema;
use diesel::prelude::*;
use diesel::MysqlConnection;
use mockall::automock;
use schema::settings::dsl::*;

#[automock]
pub trait SettingsRepository {
    fn get(
        &self,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<Setting>, diesel::result::Error>;
}

pub struct RealSettingsRepository;

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
