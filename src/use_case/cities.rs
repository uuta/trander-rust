use crate::repository::google_place_ids::GooglePlaceIdsRepository;
use crate::service::location;
use crate::service::location::new_angle::NewAngleService;
use crate::service::location::new_dest::NewDestService;
use diesel::MysqlConnection;
use geo::Point;
use mockall::automock;

#[automock]
pub trait CitiesUseCase<R: GooglePlaceIdsRepository> {
    fn get(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
        p: GetParams,
    ) -> Result<(), diesel::result::Error>;
}

pub struct ImplCitiesUseCase;

pub struct GetParams {
    lng: f64,
    lat: f64,
    distance: f64,
    direction_type: location::DirectionType,
}

impl<R: GooglePlaceIdsRepository> CitiesUseCase<R> for ImplCitiesUseCase {
    fn get(
        &self,
        repo: &R,
        conn: &mut MysqlConnection,
        p: GetParams,
    ) -> Result<(), diesel::result::Error> {
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
        let (lat, lng) = location_service.location();
    }
}
