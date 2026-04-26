use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    humidity: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Weather app in rust");
    // Get command line argument for city
    let args: Vec<String> = env::args().collect();
    let city = &args[1];
    dotenv().ok();
    // Get env data
    let api_key = env::var("API_KEY").expect("API_KEY does not exists!");

    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}");
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    // match res.status() {
    //     reqwest::StatusCode::OK => {
    //         match res>().await {
    //             Ok(parsed) => println!("works"),
    //             Err(_) => println!("Something went wrong"),
    //         };
    //     }
    // }
    let w: WeatherResponse = res.json().await?;
    print_funny_weather(&w);
    Ok(())
}

fn print_funny_weather(w: &WeatherResponse) {
    let temp_c = w.main.temp - 273.15;
    let feels_c = w.main.feels_like - 273.15;

    let description = w
        .weather
        .get(0)
        .map(|x| x.description.as_str())
        .unwrap_or("mysterious sky nonsense");

    let mood = if temp_c >= 30.0 {
        "🔥 The pavement is trying to cook you."
    } else if temp_c >= 20.0 {
        "😎 Pretty nice. The sun is behaving... for now."
    } else if temp_c >= 10.0 {
        "🧥 Slightly chilly. Hoodie goblin approved."
    } else {
        "🥶 Absolutely illegal temperature."
    };

    let wind_comment = if w.wind.speed < 2.0 {
        "air is basically buffering"
    } else if w.wind.speed < 6.0 {
        "gentle breeze doing side quests"
    } else {
        "wind is personally attacking your hairstyle"
    };

    println!();
    println!("╔════════════════════════════════════════════╗");
    println!("║        🧙 WEATHER GOBLIN REPORT 🧙        ║");
    println!("╠════════════════════════════════════════════╣");
    println!("║ City:        {:<28} ║", w.name);
    println!("║ Sky Drama:   {:<28} ║", description);
    println!("║ Temp:        {:>6.1}°C                    ║", temp_c);
    println!("║ Feels Like:  {:>6.1}°C                    ║", feels_c);
    println!(
        "║ Humidity:    {:>6.0}%                      ║",
        w.main.humidity
    );
    println!(
        "║ Wind:        {:>6.1} m/s                  ║",
        w.wind.speed
    );
    println!("╠════════════════════════════════════════════╣");
    println!("║ {:<42} ║", mood);
    println!("║ Wind status: {:<29} ║", wind_comment);
    println!("╚════════════════════════════════════════════╝");
    println!();
}
