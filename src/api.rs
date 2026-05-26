use std::error::Error;

use crate::model::{GeocodingResponse, Location, WeatherResponse};

pub async fn fetch_location(
    client: &reqwest::Client,
    city: &str,
) -> Result<Location, Box<dyn Error>> {
    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={city}&count=1&language=en&format=json"
    );

    let geocoding_res = client.get(geocoding_url).send().await?;
    let geocoding = geocoding_res.json::<GeocodingResponse>().await?;
    let location = geocoding
        .results
        .and_then(|mut results| results.pop())
        .expect("City was not found");

    Ok(location)
}

pub async fn fetch_weather(
    client: &reqwest::Client,
    location: &Location,
) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,weather_code,wind_speed_10m&timezone=auto",
        location.latitude, location.longitude,
    );

    let res = client.get(url).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json::<WeatherResponse>().await?),
        other => {
            panic!("Other error! {:?}", other);
        }
    }
}
