use std::fs;
use std::fs::File;
use std::io::{self, Write};

use crate::model::{LegacySavedCity, SavedCities};

const CITIES_FILE: &str = "cities.json";

pub fn save_city(city: &str) -> io::Result<()> {
    let mut cities = read_saved_cities().unwrap_or_default();

    if !cities.iter().any(|saved_city| saved_city == city) {
        cities.push(city.to_string());
    }

    write_saved_cities(cities)
}

pub fn delete_city_by_number(number: usize) -> io::Result<String> {
    if number == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "City numbers start from 1",
        ));
    }

    let mut cities = read_saved_cities()?;
    if number > cities.len() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No saved city found for number {number}"),
        ));
    }

    let removed_city = cities.remove(number - 1);
    write_saved_cities(cities)?;

    Ok(removed_city)
}

pub fn get_city_by_number(number: usize) -> io::Result<String> {
    if number == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "City numbers start from 1",
        ));
    }

    let cities = read_saved_cities()?;
    cities.get(number - 1).cloned().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("No saved city found for number {number}"),
        )
    })
}

pub fn saved_cities() -> io::Result<Vec<String>> {
    read_saved_cities()
}

pub fn read_saved_cities() -> io::Result<Vec<String>> {
    let json_file = fs::read_to_string(CITIES_FILE)?;

    if let Ok(saved_cities) = serde_json::from_str::<SavedCities>(&json_file) {
        return Ok(saved_cities.cities);
    }

    let legacy_city: LegacySavedCity =
        serde_json::from_str(&json_file).map_err(io::Error::other)?;
    Ok(vec![legacy_city.city])
}

fn write_saved_cities(cities: Vec<String>) -> io::Result<()> {
    let saved_cities = SavedCities { cities };
    let json_data = serde_json::to_string_pretty(&saved_cities).map_err(io::Error::other)?;
    let mut file = File::create(CITIES_FILE)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
