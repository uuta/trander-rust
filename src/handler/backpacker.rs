use crate::db;
use crate::error::http_error::HttpError;
use crate::error_log;
use crate::from_request::backpacker::Params;
use crate::info_request_log;
use crate::repository::google_place_ids::ImplGooglePlaceIdsRepository;
use crate::use_case::backpacker::{BackpackerUseCase, ImplBackpackerUseCase};
use actix_web::web::{Data, Json};
use actix_web::{get, Responder};
use tracing::error;
use tracing::info;

#[get("/backpacker")]
pub async fn get(db: Data<db::DbPool>, params: Params) -> Result<impl Responder, HttpError> {
    info_request_log!();
    let conn_result = db.get();
    let mut conn = match conn_result {
        Ok(conn) => conn,
        Err(e) => {
            error_log!(e);
            return Err(HttpError::from(e));
        }
    };
    let repo = ImplGooglePlaceIdsRepository;
    let use_case = ImplBackpackerUseCase;
    let result = use_case.get(&repo, &mut conn, params).await;
    match result {
        Ok(r) => Ok(Json(r)),
        Err(e) => {
            error_log!(e);
            Err(HttpError::from(e))
        }
    }
}
