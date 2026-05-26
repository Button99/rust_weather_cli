use std::env;
use std::io;

use crate::{api, display, storage};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("You must specify a city");
        println!("e.g. Athens");
        println!("Or list saved cities with: ls");
        println!("Or delete a saved city with: delete 1");
        return Ok(());
    }

    if args[1] == "ls" || args[1] == "--ls" {
        print_saved_cities()?;
        return Ok(());
    }

    if args[1] == "delete" || args[1] == "del" || args[1] == "rm" {
        delete_saved_city(&args)?;
        return Ok(());
    }

    let city = if let Ok(saved_city_number) = args[1].parse::<usize>() {
        storage::get_city_by_number(saved_city_number)?
    } else {
        args[1].clone()
    };

    let client = reqwest::Client::new();
    print_weather_for_city(&client, &city).await?;
    ask_to_save_city(&city)?;

    Ok(())
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

fn delete_saved_city(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let Some(city_number) = args.get(2) else {
        println!("You must specify a saved city number to delete");
        println!("e.g. delete 1");
        return Ok(());
    };

    let city_number = city_number.parse::<usize>()?;
    let removed_city = storage::delete_city_by_number(city_number)?;
    println!("Deleted saved city: {removed_city}");

    Ok(())
}

async fn print_weather_for_city(
    client: &reqwest::Client,
    city: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let location = api::fetch_location(client, city).await?;
    let weather = api::fetch_weather(client, &location).await?;

    display::print_funny_weather(&weather, &location.name);

    Ok(())
}

fn ask_to_save_city(city: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Would you like to Save the city for future reference? (y/n)");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let option = input.trim().to_lowercase();

            if option == "y" {
                storage::save_city(city)?;
            } else if option == "n" {
                println!("Ok, bye!");
            }
        }
        Err(err) => println!("error: {err}"),
    }

    Ok(())
}
