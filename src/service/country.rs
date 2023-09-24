use crate::error::http_error::{HttpError, HttpErrorType};
use mockall::automock;
use rand;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Country {
    name: String,
    region: String,
    timezones: HashMap<String, String>,
    iso: Iso,
    phone: Vec<String>,
    emoji: String,
    image: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Iso {
    #[serde(rename = "alpha-2")]
    alpha_2: String,
    #[serde(rename = "alpha-3")]
    alpha_3: String,
    numeric: String,
}

#[automock]
pub trait CountryService {
    fn rnd(&mut self) -> Result<String, HttpError>;
}

pub struct ImplCountryService {
    countries: HashMap<String, Country>,
}

impl ImplCountryService {
    pub fn new() -> Result<Self, HttpError> {
        // Read the contents of the file into a string
        let file_contents = fs::read_to_string("src/service/country/country.json")
            .map_err(|e| HttpError::from(e))?;

        // Deserialize the JSON into a HashMap
        let countries: HashMap<String, Country> =
            serde_json::from_str(&file_contents).map_err(|e| HttpError::from(e))?;
        Ok(Self { countries })
    }
}

impl CountryService for ImplCountryService {
    fn rnd(&mut self) -> Result<String, HttpError> {
        let mut rng = rand::thread_rng();
        let binding = self.countries.iter().collect::<Vec<_>>();
        let random_entry = binding.choose(&mut rng);
        match random_entry {
            Some((key, _)) => Ok(key.to_string()),
            None => Err(HttpError {
                cause: None,
                message: Some("Item not found".to_string()),
                error_type: HttpErrorType::NotFoundError,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test the mocked rnd function
    fn test_rnd() {
        let mut mock = MockCountryService::new();
        mock.expect_rnd()
            .returning(|| Ok("DE".to_string()))
            .times(1);
        let result = mock.rnd();
        assert_eq!(result.unwrap(), "DE".to_string());
    }
}
