use crate::db;
use crate::error::http_error::HttpError;
use crate::from_request::cities::GetParams;
use crate::repository::google_place_ids::ImplGooglePlaceIdsRepository;
use crate::use_case::cities::{CitiesUseCase, ImplCitiesUseCase};
use actix_web::web::{Data, Json};
use actix_web::{get, Responder};

#[get("/cities")]
pub async fn get(db: Data<db::DbPool>, params: GetParams) -> Result<impl Responder, HttpError> {
    let mut conn = db
        .get()
        .map_err(|_| HttpError::new("DatabaseError", "Error getting connection".to_string()))?;
    let repo = ImplGooglePlaceIdsRepository;
    let service = ImplCitiesUseCase;
    let result = service.get(&repo, &mut conn, params).await;
    match result {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(HttpError::new("RequestError", e.to_string())),
    }
}
