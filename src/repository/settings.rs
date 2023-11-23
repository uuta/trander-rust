use crate::from_request::settings::{AddParams, UpdateParams};
use crate::model::setting::{NewSetting, Setting};
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
        params: UpdateParams,
    ) -> Result<usize, diesel::result::Error>;

    fn add(
        &self,
        conn: &mut MysqlConnection,
        user_id_value: &u64,
        params: &AddParams,
    ) -> Result<usize, diesel::result::Error>;
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
        params: UpdateParams,
    ) -> Result<usize, diesel::result::Error> {
        let count = diesel::update(settings.filter(user_id.eq(user_id_value)))
            .set((
                min_distance.eq(params.min_distance),
                max_distance.eq(params.max_distance),
                direction_type.eq(params.direction_type),
            ))
            .execute(conn)?;
        Ok(count)
    }

    fn add(
        &self,
        conn: &mut MysqlConnection,
        user_id_value: &u64,
        params: &AddParams,
    ) -> Result<usize, diesel::result::Error> {
        let insert = NewSetting {
            user_id: *user_id_value,
            min_distance: params.min_distance,
            max_distance: params.max_distance,
            direction_type: params.direction_type,
        };
        diesel::insert_into(settings).values(&insert).execute(conn)
    }
}
