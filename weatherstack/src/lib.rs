use reqwest::blocking::Client;
use serde::Deserialize;
use thiserror::Error;

const API_HOST: &str = "http://api.weatherstack.com";

#[derive(Debug, Error)]
pub enum WeatherStackError {
    #[error("api error: {0}")]
    ApiError(String),

    #[error("received an error from the API")]
    ErrorResponse(ErrorResponse),

    #[error("an unknown error: {0}")]
    Unknown(String),
}

#[derive(Debug, Deserialize)]
pub struct WeatherStackErrorData {
    pub code: usize,
    pub info: String,

    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: WeatherStackErrorData,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub typ: String,
    pub query: String,
    pub language: String,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub region: String,
    pub lat: String,
    pub lon: String,
    pub timezone_id: String,
    pub localtime: String,
    pub localtime_epoch: usize,
    pub utc_offset: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub observation_time: String,
    pub temperature: i8,
    pub weather_code: usize,
    pub weather_icons: Vec<String>,
    pub weather_descriptions: Vec<String>,
    pub wind_speed: usize,
    pub wind_degree: usize,
    pub wind_dir: String,
    pub pressure: usize,
    pub precip: usize,
    pub humidity: usize,
    pub cloudcover: usize,
    pub feelslike: i8,
    pub uv_index: usize,
    pub visibility: usize,
}

#[derive(Debug, Deserialize)]
pub struct SuccessfulResponse {
    request: Request,
    location: Location,
    current: Current,
}

#[derive(Debug)]
pub struct WeatherStack {
    api_key: String,
    client: Client,
}

impl WeatherStack {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn current(&self, query: String) -> Result<SuccessfulResponse, WeatherStackError> {
        let endpoint = format!("{}/current", API_HOST);
        let request = self
            .client
            .get(&endpoint)
            .query(&[("access_key", &self.api_key), ("query", &query)]);

        let response = request
            .send()
            .map_err(|e| WeatherStackError::ApiError(e.to_string()))?;

        let data = response
            .text()
            .map_err(|e| WeatherStackError::ApiError(e.to_string()))?;

        let response = serde_json::from_str::<SuccessfulResponse>(&data);

        match response {
            Ok(response) => Ok(response),
            Err(_) => {
                let err: ErrorResponse = serde_json::from_str(&data)
                    .map_err(|e| WeatherStackError::Unknown(e.to_string()))?;
                Err(WeatherStackError::ErrorResponse(err))
            }
        }
    }
}
