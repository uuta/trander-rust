use crate::error::http_error::HttpError;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::Client;
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

async fn cities(location: &str) -> Result<Data, HttpError> {
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

    let params = [("location", location), ("limit", "1"), ("radius", "100")];
    let client = Client::default();
    let res = client
        .get("https://wft-geo-db.p.rapidapi.com/v1/geo/cities")
        .query(&params)
        .headers(headers)
        .send()
        .await
        .map_err(|e| HttpError::new("ApiRequestError", e.to_string()))? // Network errors
        .error_for_status() // HTTP status errors
        .map_err(|e| HttpError::new("ApiStatusError", e.to_string()))?; // Convert to HttpError

    let data: String = res
        .text()
        .await
        .map_err(|e| HttpError::new("JsonParseError", e.to_string()))?;

    let parsed_data: Data =
        serde_json::from_str(&data).map_err(|e| HttpError::new("JsonParseError", e.to_string()))?;

    Ok(parsed_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::test;

    #[test]
    async fn test_cities() {
        let result = cities("+12.969576+100.900606").await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.data[0].city, "Bang Lamung");
        assert_eq!(res.data[0].country, "Thailand");
    }
}
