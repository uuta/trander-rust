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

    fn update(
        &self,
        conn: &mut MysqlConnection,
        user_id_value: u64,
        min_distance_value: i32,
        max_distance_value: i32,
        direction_type_value: i16,
    ) -> Result<(), diesel::result::Error>;
}

pub struct ImplSettingsRepository;

impl SettingsRepository for ImplSettingsRepository {
    fn get(
        &self,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<Setting>, diesel::result::Error> {
        settings
            .filter(user_id.eq(user_id_value))
            .load::<Setting>(conn)
    }

    fn update(
        &self,
        conn: &mut MysqlConnection,
        user_id_value: u64,
        min_distance_value: i32,
        max_distance_value: i32,
        direction_type_value: i16,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(settings.filter(user_id.eq(user_id_value)))
            .set((
                min_distance.eq(min_distance_value),
                max_distance.eq(max_distance_value),
                direction_type.eq(direction_type_value),
            ))
            .execute(conn)
            .map(|_| ())
    }
}
