use actix_web::{dev, web, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UpdateParams {
    pub min_distance: i32,
    pub max_distance: i32,
    pub direction_type: i16,
}

impl FromRequest for UpdateParams {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        // Get query strings from a request
        let query: web::Query<HashMap<String, String>> =
            web::Query::from_query(req.query_string()).unwrap();

        let params = UpdateParams {
            min_distance: query
                .get("min_distance")
                .unwrap_or(&"0".to_string())
                .parse::<i32>()
                .unwrap_or(0),
            max_distance: query
                .get("max_distance")
                .unwrap_or(&"0".to_string())
                .parse::<i32>()
                .unwrap_or(0),
            direction_type: query
                .get("direction_type")
                .unwrap_or(&"0".to_string())
                .parse::<i16>()
                .unwrap_or(0),
        };

        ok(params)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AddParams {
    pub min_distance: i32,
    pub max_distance: i32,
    pub direction_type: i16,
}
