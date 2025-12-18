use std::time::Duration;

/// NOAA Aviation Weather base URL
const NOAA_BASE_URL: &str = "https://aviationweather.gov/api/data";

/// Fetch raw METAR string for given ICAO
pub fn fetch_metar(icao: &str) -> Result<String, FetchError> {
    let url = format!("{}/metar?ids={}&format=raw", NOAA_BASE_URL, icao);

    fetch(&url)
}

/// Fetch raw TAF string for given ICAO
pub fn fetch_taf(icao: &str) -> Result<String, FetchError> {
    let url = format!("{}/taf?ids={}&format=raw", NOAA_BASE_URL, icao);

    fetch(&url)
}

/// Shared HTTP fetch logic
fn fetch(url: &str) -> Result<String, FetchError> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(FetchError::HttpClient)?;

    let response = client.get(url).send().map_err(FetchError::Request)?;

    if !response.status().is_success() {
        return Err(FetchError::HttpStatus(response.status().as_u16()));
    }

    let body = response.text().map_err(FetchError::Body)?;

    let trimmed = body.trim();
    if trimmed.is_empty() {
        return Err(FetchError::EmptyResponse);
    }

    Ok(trimmed.to_string())
}

/// Error type for fetch operations
#[derive(Debug)]
pub enum FetchError {
    HttpClient(reqwest::Error),
    Request(reqwest::Error),
    Body(reqwest::Error),
    HttpStatus(u16),
    EmptyResponse,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::HttpClient(e) => write!(f, "HTTP client error: {}", e),
            FetchError::Request(e) => write!(f, "HTTP request failed: {}", e),
            FetchError::Body(e) => write!(f, "Failed to read response body: {}", e),
            FetchError::HttpStatus(code) => {
                write!(f, "HTTP request failed with status {}", code)
            }
            FetchError::EmptyResponse => write!(f, "Empty response from server"),
        }
    }
}

impl std::error::Error for FetchError {}
