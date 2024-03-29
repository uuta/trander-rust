use crate::service::location;
use actix_web::{dev, web, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct GetParams {
    pub lng: f64,
    pub lat: f64,
    pub min_distance: f64,
    pub max_distance: f64,
    pub direction_type: location::DirectionType,
    pub keyword: String,
}

impl FromRequest for GetParams {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        // Get query strings from a request
        let query: web::Query<HashMap<String, String>> =
            web::Query::from_query(req.query_string()).unwrap();

        let params = GetParams {
            lng: query
                .get("lng")
                .unwrap_or(&"0.0".to_string())
                .parse::<f64>()
                .unwrap_or(0.0),
            lat: query
                .get("lat")
                .unwrap_or(&"0.0".to_string())
                .parse::<f64>()
                .unwrap_or(0.0),
            min_distance: query
                .get("min")
                .unwrap_or(&"0.0".to_string())
                .parse::<f64>()
                .map(|d| d * 1000.0)
                .unwrap_or(0.0),
            max_distance: query
                .get("max")
                .unwrap_or(&"0.0".to_string())
                .parse::<f64>()
                .map(|d| d * 1000.0)
                .unwrap_or(0.0),
            direction_type: query
                .get("directionType")
                .and_then(|s| s.parse().ok())
                .unwrap_or(location::DirectionType::All),
            keyword: query.get("keyword").unwrap_or(&"".to_string()).to_string(),
        };

        ok(params)
    }
}
