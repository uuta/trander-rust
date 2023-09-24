use actix_web::{dev, web, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Params {
    pub keyword: String,
}

impl FromRequest for Params {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        // Get query strings from a request
        let query: web::Query<HashMap<String, String>> =
            web::Query::from_query(req.query_string()).unwrap();

        let params = Params {
            keyword: query.get("keyword").unwrap_or(&"".to_string()).to_string(),
        };

        ok(params)
    }
}
