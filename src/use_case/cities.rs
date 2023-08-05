use crate::api::api_handler::ImplApiHandler;
use crate::api::geo_db_cities::geo_db_cities;
use crate::api::near_by_search::near_by_search;
use crate::error::http_error::HttpError;
use crate::repository::google_place_ids::GooglePlaceIdsRepository;
use crate::service::location;
use crate::service::location::new_angle::NewAngleService;
use crate::service::location::new_dest::NewDestService;
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
    ) -> Result<(), HttpError>;
}

pub struct ImplCitiesUseCase;

pub struct GetParams {
    lng: f64,
    lat: f64,
    distance: f64,
    direction_type: location::DirectionType,
}

#[async_trait]
impl<R: GooglePlaceIdsRepository> CitiesUseCase<R> for ImplCitiesUseCase {
    async fn get(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
        p: GetParams,
    ) -> Result<(), HttpError> {
        let mut location_service = location::ImplLocationService::new(
            p.lng,
            p.lat,
            p.distance,
            p.direction_type,
            Box::new(NewAngleService),
            Box::new(NewDestService {
                dest: Point::new(0.0, 0.0),
            }),
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
            is_ok(first) => {
                // google_place_idsテーブルにデータを挿入
                // TODO: near_by_search_dataの最初の要素を取得し、google_place_idsテーブルに挿入する
                repo.upsert(conn, near_by_search_data.upsert_params(first))
                    .await?;
                // TODO: near_by_search_dataの最初の要素を返す
            }
            _ => {
                return Err(HttpError::new(
                    "NotFound",
                    "near_by_search_data.first() is None".to_string(),
                ));
            }
        }
    }
}
