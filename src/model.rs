use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "rust_weather_cli")]
#[command(about = "get current weather")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// City name or number of saved city
    pub city: Option<String>,

    /// Save city without prompting
    #[arg(long)]
    pub save: bool,

    #[arg(long)]
    pub no_save: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Ls,
    Delete { number: usize },
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saved_cities_roundtrip() {
        let original = SavedCities {
            cities: vec!["Athens".to_string(), "Copenhagen".to_string()],
        };

        let json = serde_json::to_string_pretty(&original).unwrap();
        let deserialized: SavedCities = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.cities, original.cities);
    }

    #[test]
    fn test_saved_cities_empty() {
        let original = SavedCities { cities: vec![] };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SavedCities = serde_json::from_str(&json).unwrap();

        assert!(deserialized.cities.is_empty());
    }

    #[test]
    fn test_legacy_saved_city_deserialize() {
        let json = r#"{"city": "Athens"}"#;
        let legacy: LegacySavedCity = serde_json::from_str(json).unwrap();

        assert_eq!(legacy.city, "Athens");
    }

    #[test]
    fn test_geocoding_response_no_results() {
        let json = r#"{"results": null}"#;
        let response: GeocodingResponse = serde_json::from_str(json).unwrap();

        assert!(response.results.is_none());
    }

    #[test]
    fn test_geocoding_response_with_results() {
        let json = r#"{
            "results": [
                {"name": "Athens", "latitude": 37.98, "longitude": 23.73}
            ]
        }"#;
        let response: GeocodingResponse = serde_json::from_str(json).unwrap();

        let results = response.results.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Athens");
        assert!((results[0].latitude - 37.98).abs() < f64::EPSILON);
        assert!((results[0].longitude - 23.73).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cli_parses_city_arg() {
        let cli = Cli::try_parse_from(["rust_weather_cli", "Athens"]).unwrap();

        assert!(cli.command.is_none());
        assert_eq!(cli.city.as_deref(), Some("Athens"));
        assert!(!cli.save);
        assert!(!cli.no_save);
    }

    #[test]
    fn test_cli_parses_saved_city_number_as_city_arg() {
        let cli = Cli::try_parse_from(["rust_weather_cli", "1"]).unwrap();

        assert!(cli.command.is_none());
        assert_eq!(cli.city.as_deref(), Some("1"));
    }

    #[test]
    fn test_cli_parses_save_flags() {
        let save_cli = Cli::try_parse_from(["rust_weather_cli", "Athens", "--save"]).unwrap();
        let no_save_cli = Cli::try_parse_from(["rust_weather_cli", "Athens", "--no-save"]).unwrap();

        assert!(save_cli.save);
        assert!(!save_cli.no_save);
        assert!(!no_save_cli.save);
        assert!(no_save_cli.no_save);
    }

    #[test]
    fn test_cli_parses_ls_subcommand() {
        let cli = Cli::try_parse_from(["rust_weather_cli", "ls"]).unwrap();

        assert!(matches!(cli.command, Some(Command::Ls)));
        assert!(cli.city.is_none());
    }

    #[test]
    fn test_cli_parses_delete_subcommand() {
        let cli = Cli::try_parse_from(["rust_weather_cli", "delete", "2"]).unwrap();

        match cli.command {
            Some(Command::Delete { number }) => assert_eq!(number, 2),
            other => panic!("expected delete command, got {other:?}"),
        }
        assert!(cli.city.is_none());
    }

    #[test]
    fn test_weather_response_deserialize() {
        let json = r#"{
            "current": {
                "temperature_2m": 22.5,
                "apparent_temperature": 20.1,
                "relative_humidity_2m": 65.0,
                "weather_code": 2,
                "wind_speed_10m": 12.3
            },
            "current_units": {
                "temperature_2m": "°C",
                "apparent_temperature": "°C",
                "relative_humidity_2m": "%",
                "wind_speed_10m": "km/h"
            }
        }"#;
        let weather: WeatherResponse = serde_json::from_str(json).unwrap();

        assert!((weather.current.temperature_2m - 22.5).abs() < f64::EPSILON);
        assert!((weather.current.apparent_temperature - 20.1).abs() < f64::EPSILON);
        assert!((weather.current.relative_humidity_2m - 65.0).abs() < f64::EPSILON);
        assert_eq!(weather.current.weather_code, 2);
        assert!((weather.current.wind_speed_10m - 12.3).abs() < f64::EPSILON);

        assert_eq!(weather.current_units.temperature_2m, "°C");
        assert_eq!(weather.current_units.apparent_temperature, "°C");
        assert_eq!(weather.current_units.relative_humidity_2m, "%");
        assert_eq!(weather.current_units.wind_speed_10m, "km/h");
    }
}
