use crate::api::api_handler::get_handler;
use crate::error::http_error::HttpError;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct City {
    id: i32,
    wiki_data_id: Option<String>,
    city: String,
    name: String,
    country: String,
    country_code: String,
    region: Option<String>,
    region_code: Option<String>,
    region_wd_id: Option<String>,
    latitude: f64,
    longitude: f64,
    population: Option<i32>,
    distance: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct Data {
    data: Vec<City>,
}

async fn geo_db_cities(location: &str) -> Result<Data, HttpError> {
    dotenv().ok();
    let key = env::var("GEO_DB_CITIES_API_KEY").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-rapidapi-host",
        "wft-geo-db.p.rapidapi.com".parse().map_err(|_| {
            HttpError::new("InvalidHeaderValue", "Invalid header value".to_string())
        })?,
    );
    headers.insert(
        "x-rapidapi-key",
        key.parse().map_err(|_| {
            HttpError::new("InvalidHeaderValue", "Invalid header value".to_string())
        })?,
    );

    let params = [
        ("location", location.to_string()),
        ("limit", "1".to_string()),
        ("radius", "100".to_string()),
    ];
    let data = get_handler(
        "https://wft-geo-db.p.rapidapi.com/v1/geo/cities",
        headers,
        params.to_vec(),
    )
    .await?;

    let parsed_data: Data =
        serde_json::from_str(&data).map_err(|e| HttpError::new("JsonParseError", e.to_string()))?;

    Ok(parsed_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::test;

    #[test]
    async fn test_geo_db_cities() {
        let result = geo_db_cities("+12.969576+100.900606").await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.data[0].city, "Bang Lamung");
        assert_eq!(res.data[0].country, "Thailand");
    }
}
