use crate::error::http_error::HttpError;
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::Client;

use mockall::{automock, predicate::*};

#[automock]
#[async_trait]
pub trait ApiHandler {
    async fn get_handler(
        &self,
        end_point: &str,
        headers: HeaderMap,
        params: Vec<(String, String)>,
    ) -> Result<String, HttpError>;
}

struct RealApiHandler;

#[async_trait]
impl ApiHandler for RealApiHandler {
    async fn get_handler(
        &self,
        end_point: &str,
        headers: HeaderMap,
        params: Vec<(String, String)>,
    ) -> Result<String, HttpError> {
        dotenv().ok();

        let client = Client::default();
        let res = client
            .get(end_point)
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

        Ok(data)
    }
}
