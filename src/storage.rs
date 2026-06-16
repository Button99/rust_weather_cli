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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use tempfile::TempDir;

    static STORAGE_LOCK: Mutex<()> = Mutex::new(());

    struct CwdGuard {
        original_dir: PathBuf,
        _temp_dir: TempDir,
    }

    impl CwdGuard {
        fn new() -> Self {
            let original_dir = std::env::current_dir().expect("failed to get current dir");
            let temp_dir = TempDir::new().expect("failed to create temp dir");
            std::env::set_current_dir(temp_dir.path()).expect("failed to change cwd");
            CwdGuard {
                original_dir,
                _temp_dir: temp_dir,
            }
        }
    }

    impl Drop for CwdGuard {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.original_dir).expect("failed to restore cwd");
        }
    }

    #[test]
    fn test_save_and_list_cities() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        save_city("Athens").unwrap();
        save_city("Copenhagen").unwrap();

        let cities = saved_cities().unwrap();
        assert_eq!(cities, vec!["Athens", "Copenhagen"]);
    }

    #[test]
    fn test_save_duplicate_city() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        save_city("Athens").unwrap();
        save_city("Athens").unwrap();

        let cities = saved_cities().unwrap();
        assert_eq!(cities, vec!["Athens"]);
    }

    #[test]
    fn test_save_city_writes_current_format() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        save_city("Athens").unwrap();

        let content = fs::read_to_string(CITIES_FILE).unwrap();
        let parsed: SavedCities = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.cities, vec!["Athens"]);
        assert!(!content.contains("\"city\""));
    }

    #[test]
    fn test_get_city_by_number() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        save_city("Athens").unwrap();
        save_city("Copenhagen").unwrap();

        assert_eq!(get_city_by_number(1).unwrap(), "Athens");
        assert_eq!(get_city_by_number(2).unwrap(), "Copenhagen");
    }

    #[test]
    fn test_get_city_by_number_zero() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        let err = get_city_by_number(0).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_get_city_by_number_out_of_bounds() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        save_city("Athens").unwrap();

        let err = get_city_by_number(2).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_delete_city_by_number() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        save_city("Athens").unwrap();
        save_city("Copenhagen").unwrap();
        save_city("London").unwrap();

        let removed = delete_city_by_number(2).unwrap();
        assert_eq!(removed, "Copenhagen");

        let cities = saved_cities().unwrap();
        assert_eq!(cities, vec!["Athens", "London"]);
    }

    #[test]
    fn test_delete_city_by_number_zero() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        let err = delete_city_by_number(0).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_delete_city_by_number_out_of_bounds() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        save_city("Athens").unwrap();

        let err = delete_city_by_number(5).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_saved_cities_empty_when_no_file() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        let err = saved_cities().unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_read_saved_cities_rejects_malformed_json() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        fs::write(CITIES_FILE, "not json").unwrap();

        let err = read_saved_cities().unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Other);
    }

    #[test]
    fn test_read_legacy_format() {
        let _lock = STORAGE_LOCK.lock().unwrap();
        let _guard = CwdGuard::new();

        let legacy = r#"{"city": "Athens"}"#;
        fs::write(CITIES_FILE, legacy).unwrap();

        let cities = saved_cities().unwrap();
        assert_eq!(cities, vec!["Athens"]);

        save_city("Copenhagen").unwrap();

        let cities = saved_cities().unwrap();
        assert_eq!(cities, vec!["Athens", "Copenhagen"]);

        let content = fs::read_to_string(CITIES_FILE).unwrap();
        let parsed: SavedCities = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.cities, vec!["Athens", "Copenhagen"]);
    }
}
