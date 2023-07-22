use crate::api::api_handler::ApiHandler;
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

async fn geo_db_cities<A: ApiHandler + Send + Sync>(
    api: &A,
    location: &str,
) -> Result<Data, HttpError> {
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
        ("location".to_string(), location.to_string()),
        ("limit".to_string(), "1".to_string()),
        ("radius".to_string(), "100".to_string()),
    ];
    let data = api
        .get_handler(
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
    use crate::api::api_handler::MockApiHandler;
    use actix_rt::test;

    #[test]
    async fn test_geo_db_cities() {
        let mut mock_api = MockApiHandler::new();
        mock_api.expect_get_handler().returning(|_, _, _| {
            Ok(r#"{
                "data": [{
                    "id": 3350606,
                    "wikiDataId": "Q24668",
                    "type": "CITY",
                    "city": "Aixirivall",
                    "name": "Aixirivall",
                    "country": "Andorra",
                    "countryCode": "AD",
                    "region": "Sant Julià de Lòria",
                    "regionCode": "06",
                    "regionWdId": "Q24282",
                    "latitude": 42.46245,
                    "longitude": 1.50209,
                    "population": 0
                }]
            }"#
            .to_string())
        });
        let result = geo_db_cities(&mock_api, "+42.46245+1.50209").await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.data[0].city, "Aixirivall");
        assert_eq!(res.data[0].country, "Andorra");
        assert_eq!(res.data[0].country_code, "AD");
        assert_eq!(res.data[0].region, Some("Sant Julià de Lòria".to_string()));
    }
}
