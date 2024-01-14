use crate::api::geo_db_cities::City;
use crate::api::near_by_search::ResultItem as NearBySearchResultItem;
use crate::service::location::{ImplLocationService, LocationService};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub place_id: String,
    pub name: String,
    pub distance: f64,
    pub direction: String,
    pub country_code: String,
    pub icon: String,
    pub rating: f64,
    pub vicinity: String,
    pub user_ratings_total: i64,
    pub price_level: i64,
    pub lat: f64,
    pub lng: f64,
}

impl Response {
    pub fn new(
        cities: &City,
        near_by_search: &NearBySearchResultItem,
        location_service: &mut ImplLocationService,
        lng: f64,
        lat: f64,
    ) -> Self {
        Self {
            place_id: near_by_search.place_id(),
            name: near_by_search.name(),
            distance: location_service.distance(lng, lat),
            direction: location_service.detailed_direction().to_string(),
            country_code: cities.country_code(),
            icon: near_by_search.icon(),
            rating: near_by_search.rating(),
            vicinity: near_by_search.vicinity(),
            user_ratings_total: near_by_search.user_ratings_total(),
            price_level: near_by_search.price_level(),
            lat: near_by_search.lat(),
            lng: near_by_search.lng(),
        }
    }
}
