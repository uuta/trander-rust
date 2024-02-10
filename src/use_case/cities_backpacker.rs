use crate::api::api_handler::ImplApiHandler;
use crate::api::geo_db_cities::geo_db_cities_by_country_code;
use crate::api::near_by_search::near_by_search;
use crate::error::http_error::{HttpError, HttpErrorType};
use crate::repository::google_place_ids::GooglePlaceIdsRepository;
use crate::response;
use crate::service::country::{CountryService, ImplCountryService};
use crate::service::location;
use crate::service::location::new_angle::NewAngleService;
use crate::service::location::new_dest::NewDestService;
use crate::service::location::new_distance::NewDistanceService;
use crate::service::location::LocationService;
use crate::util::generate_random_char;
use async_trait::async_trait;
use diesel::MysqlConnection;
use geo::Point;
use mockall::automock;

#[async_trait]
#[automock]
pub trait CitiesBackpackerUseCase<R: GooglePlaceIdsRepository> {
    async fn get(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
    ) -> Result<response::backpacker::Response, HttpError>;
}

pub struct ImplCitiesBackpackerUseCase;

#[async_trait]
impl<R: GooglePlaceIdsRepository + Send + Sync> CitiesBackpackerUseCase<R>
    for ImplCitiesBackpackerUseCase
{
    async fn get(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
    ) -> Result<response::backpacker::Response, HttpError> {
        let mut country_service = ImplCountryService::new()?;
        let country_code = country_service.rnd()?;
        let geo_db_cities_data = geo_db_cities_by_country_code(
            &ImplApiHandler,
            &country_code,
            &generate_random_char().to_string(),
            "100",
        )
        .await?;
        match geo_db_cities_data.rnd() {
            Some(first_geo) => {
                let mut location_service = location::ImplLocationService::new(
                    first_geo.lng(),
                    first_geo.lat(),
                    0.0,
                    0.0,
                    location::DirectionType::All,
                    Box::new(NewAngleService),
                    Box::new(NewDestService {
                        dest: Point::new(0.0, 0.0),
                    }),
                    Box::new(NewDistanceService),
                );
                location_service.gen();

                // Set a keyword
                let keyword = first_geo.city_name();

                let near_by_search_data =
                    near_by_search(&ImplApiHandler, &location_service.concat(), &keyword).await?;
                match near_by_search_data.rnd() {
                    Some(first) => {
                        // Upsert google_place_ids table
                        let _ = repo.upsert(conn, first.upsert_params());
                        Ok(response::backpacker::Response::new(
                            &first_geo,
                            first,
                            &mut location_service,
                            first.lng(),
                            first.lat(),
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
