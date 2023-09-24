use crate::api::api_handler::ApiHandler;
use crate::error::http_error::HttpError;
use dotenv::dotenv;
use rand;
use rand::seq::SliceRandom;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Data {
    data: Vec<City>,
}

impl Data {
    pub fn rnd(&self) -> Option<&City> {
        let mut rng = rand::thread_rng();
        self.data.choose(&mut rng)
    }
    pub fn first(&self) -> Option<&City> {
        self.data.first()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct City {
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

impl City {
    pub fn city_name(&self) -> String {
        let mut keyword = String::new();
        if let Some(region) = &self.region {
            keyword.push_str(region);
            keyword.push_str(" ");
        }
        keyword.push_str(&self.name);
        keyword
    }
    pub fn country_code(&self) -> String {
        self.country_code.clone()
    }
    pub fn lat(&self) -> f64 {
        self.latitude
    }
    pub fn lng(&self) -> f64 {
        self.longitude
    }
}

pub async fn geo_db_cities<A: ApiHandler + Send + Sync>(
    api: &A,
    location: &str,
) -> Result<Data, HttpError> {
    dotenv().ok();
    let key = env::var("GEO_DB_CITIES_API_KEY").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-rapidapi-host",
        "wft-geo-db.p.rapidapi.com"
            .parse()
            .map_err(|e| HttpError::from(e))?,
    );
    headers.insert(
        "x-rapidapi-key",
        key.parse().map_err(|e| HttpError::from(e))?,
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

    let parsed_data: Data = serde_json::from_str(&data).map_err(|e| HttpError::from(e))?;

    Ok(parsed_data)
}

pub async fn geo_db_cities_by_country_code<A: ApiHandler + Send + Sync>(
    api: &A,
    country_ids: &str,
    name_prefix: &str,
    limit: &str,
) -> Result<Data, HttpError> {
    dotenv().ok();
    let key = env::var("GEO_DB_CITIES_API_KEY").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-rapidapi-host",
        "wft-geo-db.p.rapidapi.com"
            .parse()
            .map_err(|e| HttpError::from(e))?,
    );
    headers.insert(
        "x-rapidapi-key",
        key.parse().map_err(|e| HttpError::from(e))?,
    );

    let params = [
        ("countryIds".to_string(), country_ids.to_string()),
        ("namePrefix".to_string(), name_prefix.to_string()),
        ("limit".to_string(), limit.to_string()),
    ];
    let data = api
        .get_handler(
            "https://wft-geo-db.p.rapidapi.com/v1/geo/cities",
            headers,
            params.to_vec(),
        )
        .await?;

    let parsed_data: Data = serde_json::from_str(&data).map_err(|e| HttpError::from(e))?;

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
        let first = res.first();
        assert_eq!(first.unwrap().city, "Aixirivall");
        assert_eq!(first.unwrap().country, "Andorra");
        assert_eq!(first.unwrap().country_code, "AD");
        assert_eq!(
            first.unwrap().region,
            Some("Sant Julià de Lòria".to_string())
        );
        let city_name = first.unwrap().city_name();
        assert_eq!(city_name, "Sant Julià de Lòria Aixirivall");
    }

    #[test]
    async fn test_geo_db_cities_by_country_code() {
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
                },
                {
                    "id":3474327,
                    "wikiDataId":"Q3596203",
                    "type":"CITY",
                    "city":"'Au'asi",
                    "name":"'Au'asi",
                    "country":"United States of America",
                    "countryCode":"US",
                    "region":"American Samoa",
                    "regionCode":"AS",
                    "regionWdId":"Q16641",
                    "latitude":-14.276666666,
                    "longitude":-170.574444444,
                    "population":125
                }]
            }"#
            .to_string())
        });
        let result = geo_db_cities_by_country_code(&mock_api, "US", "a", "100").await;
        assert!(result.is_ok());
        let res = result.unwrap();
        let first = res.first();
        assert_eq!(first.unwrap().city, "Aixirivall");
        assert_eq!(first.unwrap().country, "Andorra");
        assert_eq!(first.unwrap().country_code, "AD");
        assert_eq!(
            first.unwrap().region,
            Some("Sant Julià de Lòria".to_string())
        );
        let city_name = first.unwrap().city_name();
        assert_eq!(city_name, "Sant Julià de Lòria Aixirivall");
    }
}
