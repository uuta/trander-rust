use crate::db;
use crate::error::http_error::HttpError;
use crate::from_request::settings::UpdateParams;
use crate::repository::settings::ImplSettingsRepository;
use crate::service::settings::{ImplSettingsService, SettingsService};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, put, Responder};

#[get("/settings/{user_id}")]
pub async fn get(db: Data<db::DbPool>, path: Path<u64>) -> Result<impl Responder, HttpError> {
    let user_id = path.into_inner();
    let mut conn = db
        .get()
        .map_err(|_| HttpError::new("DatabaseError", "Error getting connection".to_string()))?;
    let repo = ImplSettingsRepository;
    let service = ImplSettingsService;
    let result = service
        .get(&repo, user_id, &mut conn)
        .map_err(|_| HttpError::new("SettingsLoadError", "Error loading settings".to_string()))?;
    Ok(Json(result))
}

#[put("/settings/{user_id}")]
pub async fn put(
    db: Data<db::DbPool>,
    path: Path<u64>,
    params: UpdateParams,
) -> Result<impl Responder, HttpError> {
    let user_id = path.into_inner();
    let mut conn = db
        .get()
        .map_err(|_| HttpError::new("DatabaseError", "Error getting connection".to_string()))?;
    let repo = ImplSettingsRepository;
    let service = ImplSettingsService;
    let result = service
        .update(&repo, &mut conn, user_id, params)
        .map_err(|_| {
            HttpError::new("SettingsUpdateError", "Error updating settings".to_string())
        })?;
    Ok(Json(result))
}
