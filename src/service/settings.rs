use crate::model::setting::Setting;
use crate::repository::settings::SettingsRepository;
use diesel::MysqlConnection;

pub fn get<R: SettingsRepository>(
    repo: &R,
    user_id_value: u64,
    conn: &mut MysqlConnection,
) -> Result<Vec<Setting>, diesel::result::Error> {
    repo.get(user_id_value, conn)
}
