use reqwest;
use serde::{ Deserialize, Serialize };
use std::env;
use std::fs::File;
use std::io::Write;
use std::io;
use std::fs;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current: Current,
    current_units: CurrentUnits,
}

#[derive(Debug, Deserialize)]
struct Current {
    temperature_2m: f64,
    apparent_temperature: f64,
    relative_humidity_2m: f64,
    weather_code: u8,
    wind_speed_10m: f64,
}

#[derive(Debug, Deserialize)]
struct CurrentUnits {
    temperature_2m: String,
    apparent_temperature: String,
    relative_humidity_2m: String,
    wind_speed_10m: String,
}

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<Location>>,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize)]
struct CityToJson<'a> {
    city: &'a str 
}
#[derive(Deserialize)]
struct JsonToCity {
    city: String 
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line argument for city
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("You must specify a city");
        println!("e.g. Athens");
    }

    let city = if args[1].parse::<u8>().is_ok() {
        getCityFromFile()
    } else {
        args[1].clone()
    };

    let client = reqwest::Client::new();

    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={city}&count=1&language=en&format=json"
    );
    let geocoding_res = client.get(geocoding_url).send().await?;
    let geocoding = geocoding_res.json::<GeocodingResponse>().await?;
    let location = geocoding
        .results
        .and_then(|mut results| results.pop())
        .expect("City was not found");

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,weather_code,wind_speed_10m&timezone=auto",
        location.latitude,
        location.longitude,
    );

    let res = client.get(url).send().await?;
    match res.status() {
        reqwest::StatusCode::OK => match &res.json::<WeatherResponse>().await {
            Ok(parsed) => print_funny_weather(parsed, &location.name),
            Err(_) => println!("Something went wrong"),
        },
        other => {
            panic!("Other error! {:?}", other);
        }
    }
    println!("Would you like to Save the city for future reference? (y/n)");
    let mut input = String::new();
 
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let option = input.trim().to_lowercase();

            if option == "y" {
                saving_city(&city)?; 
            }
            else if option == "n" {
                println!("Ok, bye!");
            } 
        }
        Err(err) => println!("error: {err}"),
    }

    Ok(())
}

fn saving_city(city: &str) -> std::io::Result<()> {
   let city_j = CityToJson {
       city: city
   };

    let json_data = serde_json::to_string_pretty(&city_j).unwrap();
    let mut file = File::create("cities.json")?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn getCityFromFile() -> String {
        let json_file = fs::read_to_string("cities.json")
        .expect("Sorry, file does not exists!");
        let fileCity: JsonToCity = serde_json::from_str(&json_file).expect("Error while performing this action");
        fileCity.city
}

fn print_funny_weather(w: &WeatherResponse, city: &str) {
    let temp_c = w.current.temperature_2m;
    let feels_c = w.current.apparent_temperature;
    let description = weather_description(w.current.weather_code);

    let mood = if temp_c >= 30.0 {
        "🔥 The pavement is trying to cook you."
    } else if temp_c >= 20.0 {
        "😎 Pretty nice. The sun is behaving... for now."
    } else if temp_c >= 10.0 {
        "🧥 Slightly chilly. Hoodie goblin approved."
    } else {
        "🥶 Absolutely illegal temperature."
    };

    let wind_comment = if w.current.wind_speed_10m < 2.0 {
        "air is basically buffering"
    } else if w.current.wind_speed_10m < 6.0 {
        "gentle breeze doing side quests"
    } else {
        "wind is personally attacking your hairstyle"
    };

    println!();
    println!("╔════════════════════════════════════════════╗");
    println!("║        🧙 WEATHER GOBLIN REPORT 🧙        ║");
    println!("╠════════════════════════════════════════════╣");
    println!("║ City:        {:<28} ║", city);
    println!("║ Sky Drama:   {:<28} ║", description);
    println!(
        "║ Temp:        {:>6.1}{}                    ║",
        temp_c,
        w.current_units.temperature_2m
    );
    println!(
        "║ Feels Like:  {:>6.1}{}                    ║",
        feels_c,
        w.current_units.apparent_temperature
    );
    println!(
        "║ Humidity:    {:>6.0}{}                      ║",
        w.current.relative_humidity_2m,
        w.current_units.relative_humidity_2m
    );
    println!(
        "║ Wind:        {:>6.1}{}                  ║",
        w.current.wind_speed_10m,
        w.current_units.wind_speed_10m
    );
    println!("╠════════════════════════════════════════════╣");
    println!("║ {:<42} ║", mood);
    println!("║ Wind status: {:<29} ║", wind_comment);
    println!("╚════════════════════════════════════════════╝");
    println!();
}

fn weather_description(code: u8) -> &'static str {
    match code {
        0 => "clear sky",
        1 | 2 | 3 => "partly cloudy",
        45 | 48 => "fog",
        51 | 53 | 55 => "drizzle",
        56 | 57 => "freezing drizzle",
        61 | 63 | 65 => "rain",
        66 | 67 => "freezing rain",
        71 | 73 | 75 => "snow",
        77 => "snow grains",
        80 | 81 | 82 => "rain showers",
        85 | 86 => "snow showers",
        95 => "thunderstorm",
        96 | 99 => "thunderstorm with hail",
        _ => "mysterious sky nonsense",
    }
}
