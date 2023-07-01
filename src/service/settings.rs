use crate::model::setting::Setting;
use crate::repository::settings::SettingsRepository;
use diesel::MysqlConnection;
use mockall::automock;

#[automock]
pub trait SettingsService<R: SettingsRepository> {
    fn get(
        &self,
        repo: &R,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<Setting>, diesel::result::Error>;
}

pub struct ImplSettingsService;

// R is a type that implements the SettingsRepository trait.
// @see https://doc.rust-jp.rs/rust-by-example-ja/generics/gen_trait.html
impl<R: SettingsRepository> SettingsService<R> for ImplSettingsService {
    fn get(
        &self,
        repo: &R,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<Setting>, diesel::result::Error> {
        repo.get(user_id_value, conn)
    }
}
