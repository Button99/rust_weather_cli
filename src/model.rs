use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub current: Current,
    pub current_units: CurrentUnits,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: f64,
    pub weather_code: u8,
    pub wind_speed_10m: f64,
}

#[derive(Debug, Deserialize)]
pub struct CurrentUnits {
    pub temperature_2m: String,
    pub apparent_temperature: String,
    pub relative_humidity_2m: String,
    pub wind_speed_10m: String,
}

#[derive(Debug, Deserialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<Location>>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize)]
pub struct LegacySavedCity {
    pub city: String,
}

#[derive(Deserialize, Serialize)]
pub struct SavedCities {
    pub cities: Vec<String>,
}
