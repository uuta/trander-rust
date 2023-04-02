use crate::db;
use crate::error::api_error::ApiError;
use crate::service::settings;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, Responder};

#[get("/settings/{user_id}")]
pub async fn get(db: Data<db::DbPool>, path: Path<u64>) -> Result<impl Responder, ApiError> {
    let user_id_value = path.into_inner();
    let mut conn = db
        .get()
        .map_err(|err| ApiError::new(format!("Error getting connection: {}", err)))?;
    let result = settings::get(user_id_value, &mut conn)
        .map_err(|err| ApiError::new(format!("Error loading settings: {}", err)))?;
    Ok(Json(result))
}
