use crate::api::api_handler::ApiHandler;
use crate::error::http_error::HttpError;
use crate::repository::google_place_ids::UpsertParams;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Data {
    results: Vec<ResultItem>,
}

impl Data {
    pub fn rand(&self) -> Option<&ResultItem> {
        use rand::seq::SliceRandom;
        self.results.choose(&mut rand::thread_rng())
    }

    pub fn first(&self) -> Result<&ResultItem, HttpError> {
        match self.results.first() {
            Some(first) => Ok(Some(first).unwrap()),
            None => Err(HttpError::new("NotFound", "City Not Found".to_string())),
        }
    }

    pub fn upsert_params(&self, res: &ResultItem) -> UpsertParams {
        UpsertParams {
            place_id: res.place_id.clone(),
            name: res.name.clone(),
            lat: res
                .geometry
                .as_ref()
                .unwrap()
                .location
                .as_ref()
                .unwrap()
                .lat,
            lng: res
                .geometry
                .as_ref()
                .unwrap()
                .location
                .as_ref()
                .unwrap()
                .lng,
            icon: res.icon.clone(),
            photo: res
                .photos
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .photo_reference
                .clone(),
            rating_star: Some(0),
            rating: res.rating,
            user_ratings_total: res.user_ratings_total,
            vicinity: res.vicinity.clone(),
            price_level: res.price_level,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ResultItem {
    business_status: Option<String>,
    geometry: Option<Geometry>,
    icon: String,
    name: String,
    place_id: String,
    rating: Option<f64>,
    user_ratings_total: Option<i64>,
    vicinity: Option<String>,
    reference: Option<String>,
    price_level: Option<i64>,
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
pub async fn near_by_search<A: ApiHandler + Send + Sync>(
    api: &A,
    location: &str,
    keyword: &str,
) -> Result<Data, HttpError> {
    dotenv().ok();
    let key = env::var("GOOGLE_PLACES_KEY").unwrap();

    let headers = HeaderMap::new();

    let params = [
        ("key".to_string(), key.to_string()),
        ("location".to_string(), location.to_string()),
        ("radius".to_string(), "10000".to_string()),
        ("keyword".to_string(), keyword.to_string()),
        ("language".to_string(), "en".to_string()),
    ];
    let data = api
        .get_handler(
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
    use crate::api::api_handler::MockApiHandler;
    use actix_rt::test;

    #[test]
    async fn test_near_by_search() {
        let mut mock_api = MockApiHandler::new();
        mock_api
            .expect_get_handler()
            .returning(|_, _, _| Ok(r#"{
        "results": [
            {
                "business_status": "OPERATIONAL",
                "geometry": {
                    "location": {
                        "lat": 25.301886,
                        "lng": 55.433433
                    },
                    "viewport": {
                        "northeast": {
                            "lat": 25.303236,
                            "lng": 55.434783
                        },
                        "southwest": {
                            "lat": 25.300536,
                            "lng": 55.432083
                        }
                    }
                },
                "icon": "https://maps.gstatic.com/mapfiles/place_api/icons/v1/png_71/lodging-71.png",
                "name": "Al Noor Island",
                "place_id": "ChIJN1t_tDeuEmsRUsoyG83frY4",
                "rating": 4.4,
                "user_ratings_total": 269,
                "vicinity": "48 Pirrama Road, Pyrmont, NSW 2009, Australia",
                "reference": "ChIJN1t_tDeuEmsRUsoyG83frY4",
                "price_level": 3,
                "photos": [
                    {
                        "height": 270,
                        "width": 519,
                        "photo_reference": "Aap_uECkX6...",
                        "html_attributions": [
                            "<a href=\"https://maps.google.com/maps/contrib/104066891898402903288\">A Google User</a>"
                        ]
                    }
                ]
            }
        ]
    }"#.to_string()));
        let result = near_by_search(&mock_api, "25.301886,55.433433", "tourist spot").await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.results[0].name, "Al Noor Island".to_string());
        assert_eq!(res.results[0].rating, Some(4.4));
        assert_eq!(res.results[0].user_ratings_total, Some(269));
    }

    #[test]
    async fn test_rand() {
        let data = Data {
            results: vec![
                ResultItem {
                    business_status: None,
                    geometry: None,
                    icon: "icon1".to_string(),
                    name: "test1".to_string(),
                    place_id: "XXXTEST1".to_string(),
                    rating: None,
                    user_ratings_total: None,
                    vicinity: None,
                    reference: None,
                    price_level: None,
                    photos: None,
                },
                ResultItem {
                    business_status: None,
                    geometry: None,
                    icon: "icon2".to_string(),
                    name: "test2".to_string(),
                    place_id: "XXXTEST2".to_string(),
                    rating: None,
                    user_ratings_total: None,
                    vicinity: None,
                    reference: None,
                    price_level: None,
                    photos: None,
                },
            ],
        };
        let rand = data.rand();
        assert!(rand.is_some());
        assert!(
            rand.unwrap().name == "test1".to_string() || rand.unwrap().name == "test2".to_string()
        );
    }

    #[test]
    async fn test_first() {
        let data = Data {
            results: vec![
                ResultItem {
                    business_status: None,
                    geometry: None,
                    icon: "icon1".to_string(),
                    name: "test1".to_string(),
                    place_id: "XXXTEST1".to_string(),
                    rating: None,
                    user_ratings_total: None,
                    vicinity: None,
                    reference: None,
                    price_level: None,
                    photos: None,
                },
                ResultItem {
                    business_status: None,
                    geometry: None,
                    icon: "icon2".to_string(),
                    name: "test2".to_string(),
                    place_id: "XXXTEST2".to_string(),
                    rating: None,
                    user_ratings_total: None,
                    vicinity: None,
                    reference: None,
                    price_level: None,
                    photos: None,
                },
            ],
        };
        let rand = data.first();
        assert!(rand.is_ok());
        assert!(rand.unwrap().name == "test1".to_string());
    }
}
