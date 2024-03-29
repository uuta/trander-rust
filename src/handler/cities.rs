use crate::db;
use crate::error::http_error::HttpError;
use crate::from_request::cities::GetParams;
use crate::info_request_log;
use crate::repository::google_place_ids::ImplGooglePlaceIdsRepository;
use crate::use_case::cities::{CitiesUseCase, ImplCitiesUseCase};
use actix_web::web::{Data, Json};
use actix_web::{get, Responder};
use tracing::info;

#[get("/cities")]
pub async fn get(db: Data<db::DbPool>, params: GetParams) -> Result<impl Responder, HttpError> {
    info_request_log!();
    let mut conn = db.get().map_err(|e| HttpError::from(e))?;
    let repo = ImplGooglePlaceIdsRepository;
    let use_case = ImplCitiesUseCase;
    let result = use_case.get(&repo, &mut conn, params).await;
    match result {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(HttpError::from(e)),
    }
}
