use crate::api::api_handler::ImplApiHandler;
use crate::api::geo_db_cities::geo_db_cities;
use crate::api::near_by_search::near_by_search;
use crate::error::http_error::{HttpError, HttpErrorType};
use crate::from_request::cities::GetParams;
use crate::repository::google_place_ids::GooglePlaceIdsRepository;
use crate::response;
use crate::service::location;
use crate::service::location::new_angle::NewAngleService;
use crate::service::location::new_dest::NewDestService;
use crate::service::location::new_distance::NewDistanceService;
use crate::service::location::LocationService;
use async_trait::async_trait;
use diesel::MysqlConnection;
use geo::Point;
use mockall::automock;

#[async_trait]
#[automock]
pub trait CitiesUseCase<R: GooglePlaceIdsRepository> {
    async fn get(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
        p: GetParams,
    ) -> Result<response::cities::Response, HttpError>;
}

pub struct ImplCitiesUseCase;

#[async_trait]
impl<R: GooglePlaceIdsRepository + Send + Sync> CitiesUseCase<R> for ImplCitiesUseCase {
    async fn get(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
        p: GetParams,
    ) -> Result<response::cities::Response, HttpError> {
        let mut location_service = location::ImplLocationService::new(
            p.lng,
            p.lat,
            p.distance,
            p.direction_type,
            Box::new(NewAngleService),
            Box::new(NewDestService {
                dest: Point::new(0.0, 0.0),
            }),
            Box::new(NewDistanceService),
        );
        location_service.gen();
        let geo_db_cities_data = geo_db_cities(&ImplApiHandler, &location_service.format()).await?;
        let near_by_search_data = near_by_search(
            &ImplApiHandler,
            &location_service.concat(),
            &geo_db_cities_data.city_name(),
        )
        .await?;
        match near_by_search_data.first() {
            Ok(first) => {
                // google_place_idsテーブルにデータを挿入
                let _ = repo.upsert(conn, near_by_search_data.upsert_params(first));
                Ok(response::cities::Response::new(
                    &geo_db_cities_data,
                    first,
                    &mut location_service,
                    p.lng,
                    p.lat,
                ))
            }
            _ => {
                return Err(HttpError {
                    cause: None,
                    message: Some("Item not found".to_string()),
                    error_type: HttpErrorType::NotFoundError,
                })
            }
        }
    }
}
