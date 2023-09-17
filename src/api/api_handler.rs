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

pub struct ImplApiHandler;

#[async_trait]
impl ApiHandler for ImplApiHandler {
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
            .map_err(|e| HttpError::from(e))?
            .error_for_status()
            .map_err(|e| HttpError::from(e))?;

        let data: String = res.text().await.map_err(|e| HttpError::from(e))?;

        Ok(data)
    }
}
