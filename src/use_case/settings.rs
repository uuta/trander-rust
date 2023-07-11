use crate::from_request::settings::UpdateParams;
use crate::model::setting::Setting;
use crate::repository::settings::SettingsRepository;
use diesel::MysqlConnection;
use mockall::automock;

#[automock]
pub trait SettingsUseCase<R: SettingsRepository> {
    fn get(
        &self,
        repo: &R,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<Setting>, diesel::result::Error>;

    fn update(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
        user_id_value: u64,
        params: UpdateParams,
    ) -> Result<(), diesel::result::Error>;
}

pub struct ImplSettingsUseCase;

// R is a type that implements the SettingsRepository trait.
// @see https://doc.rust-jp.rs/rust-by-example-ja/generics/gen_trait.html
impl<R: SettingsRepository> SettingsUseCase<R> for ImplSettingsUseCase {
    fn get(
        &self,
        repo: &R,
        user_id_value: u64,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<Setting>, diesel::result::Error> {
        repo.get(user_id_value, conn)
    }

    fn update(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
        user_id_value: u64,
        params: UpdateParams,
    ) -> Result<(), diesel::result::Error> {
        match repo.update(conn, user_id_value, params) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
