use std::io;

use clap::Parser;

use crate::model::{Cli, Command, DisplayMode};
use crate::{api, display, storage};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let save_mode = determine_save_mode(cli.save, cli.no_save);

    match cli.command {
        Some(Command::Ls) => {
            print_saved_cities()?;
            return Ok(());
        }
        Some(Command::Delete { number }) => {
            delete_saved_city(number)?;
            return Ok(());
        }
        None => {}
    }

    let Some(city_arg) = cli.city else {
        print_usage();
        return Ok(());
    };

    let city = if let Ok(n) = city_arg.parse::<usize>() {
        storage::get_city_by_number(n)?
    } else {
        city_arg
    };

    let client = reqwest::Client::new();
    let location = api::fetch_location(&client, &city).await?;
    let weather = api::fetch_weather(&client, &location).await?;

    match cli.display {
        DisplayMode::Funny => display::print_funny_weather(&weather, &location.name),
        DisplayMode::Compact => display::print_compact_weather(&weather, &location.name),
    }

    match save_mode {
        SaveMode::Yes => storage::save_city(&location.name)?,
        SaveMode::No => {}
        SaveMode::Ask => ask_to_save_city(&location.name)?,
    }

    Ok(())
}

fn print_usage() {
    println!("Weather Goblina CLI");
    println!();
    println!("Usage:");
    println!("  cargo run -- <city>           Get weather for a city");
    println!("  cargo run -- <number>         Get weather for a saved city by number");
    println!("  cargo run -- ls               List saved cities");
    println!("  cargo run -- delete <number>  Delete a saved city");
    println!("  cargo run -- <city> --save    Save city without prompting");
    println!("  cargo run -- <city> --no-save Skip save prompt");
    println!("  cargo run -- <city> --display compact");
    println!("  cargo run -- --help           Show this help");
    println!();
    println!("Examples:");
    println!("  cargo run -- Athens");
    println!("  cargo run -- Copenhagen --save");
    println!("  cargo run -- Athens --display compact");
    println!("  cargo run -- 1");
    println!("  cargo run -- ls");
    println!("  cargo run -- delete 2");
}

#[derive(Debug, PartialEq, Eq)]
enum SaveMode {
    Yes,
    No,
    Ask,
}

fn determine_save_mode(save: bool, no_save: bool) -> SaveMode {
    if save {
        SaveMode::Yes
    } else if no_save {
        SaveMode::No
    } else {
        SaveMode::Ask
    }
}

fn print_saved_cities() -> Result<(), Box<dyn std::error::Error>> {
    let cities = storage::saved_cities()?;

    if cities.is_empty() {
        println!("No saved cities found.");
        return Ok(());
    }

    println!("Saved cities:");
    for (index, city) in cities.iter().enumerate() {
        println!("{}. {}", index + 1, city);
    }

    Ok(())
}

fn delete_saved_city(num: usize) -> Result<(), Box<dyn std::error::Error>> {
    let removed_city = storage::delete_city_by_number(num)?;
    println!("Deleted saved city: {removed_city}");

    Ok(())
}

fn ask_to_save_city(city: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Would you like to save the city for future reference? (y/n)");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let option = input.trim().to_lowercase();

            if option == "y" {
                storage::save_city(city)?;
            }
        }
        Err(err) => println!("error: {err}"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_save_mode_defaults_to_ask() {
        assert_eq!(determine_save_mode(false, false), SaveMode::Ask);
    }

    #[test]
    fn test_determine_save_mode_uses_save_flag() {
        assert_eq!(determine_save_mode(true, false), SaveMode::Yes);
    }

    #[test]
    fn test_determine_save_mode_uses_no_save_flag() {
        assert_eq!(determine_save_mode(false, true), SaveMode::No);
    }

    #[test]
    fn test_determine_save_mode_save_wins_when_both_flags_are_set() {
        assert_eq!(determine_save_mode(true, true), SaveMode::Yes);
    }
}
