use crate::api::api_handler::get_handler;
use crate::error::http_error::HttpError;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Data {
    results: Vec<ResultItem>,
}

#[derive(Deserialize, Debug)]
struct ResultItem {
    business_status: Option<String>,
    geometry: Option<Geometry>,
    icon: Option<String>,
    name: String,
    place_id: Option<String>,
    rating: Option<f64>,
    user_ratings_total: Option<u32>,
    vicinity: Option<String>,
    reference: Option<String>,
    price_level: Option<u32>,
    photos: Option<Vec<Photo>>,
}

#[derive(Deserialize, Debug)]
struct Geometry {
    location: Option<Location>,
    viewport: Option<Viewport>,
}

#[derive(Deserialize, Debug)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Deserialize, Debug)]
struct Photo {
    height: Option<u32>,
    width: Option<u32>,
    photo_reference: Option<String>,
    html_attributions: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Viewport {
    northeast: Option<Location>,
    southwest: Option<Location>,
}

/// https://developers.google.com/maps/documentation/places/web-service/search-nearby#maps_http_places_nearbysearch-go
async fn near_by_search(location: &str, keyword: &str) -> Result<Data, HttpError> {
    dotenv().ok();
    let key = env::var("GOOGLE_PLACES_KEY").unwrap();

    let headers = HeaderMap::new();

    let params = [
        ("key", key),
        ("location", location.to_string()),
        ("radius", "10000".to_string()),
        ("keyword", keyword.to_string()),
        ("language", "en".to_string()),
    ];
    let data = get_handler(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json",
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
    async fn test_cities() {
        let result = near_by_search("25.301886,55.433433", "tourist spot").await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.results[0].name, "Al Noor Island".to_string());
    }
}
