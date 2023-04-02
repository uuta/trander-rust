use crate::db;
use crate::error::api_error::ApiError;
use crate::model;
use crate::schema;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, Responder};
use diesel::prelude::*;
use schema::settings::dsl::*;

#[get("/settings/{user_id}")]
pub async fn get(db: Data<db::DbPool>, path: Path<u64>) -> Result<impl Responder, ApiError> {
    let user_id_value = path.into_inner();
    let result = settings
        .filter(user_id.eq(user_id_value))
        .load::<model::Setting>(
            &mut db
                .get()
                .map_err(|err| ApiError::new(format!("Error getting connection: {}", err)))?,
        )
        .map_err(|err| ApiError::new(format!("Error loading settings: {}", err)))?;

    Ok(Json(result))
}
