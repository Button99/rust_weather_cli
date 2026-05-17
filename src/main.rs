use dotenv::dotenv;
use reqwest;
use serde::{ Deserialize, Serialize };
use std::env;
use std::fs::File;
use std::io::Write;
use std::io;
use std::fs;

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
    dotenv().ok();

    // Get env data
    let api_key = env::var("API_KEY").expect("API_KEY does not exists!");

    // 3 modes F, C, Imperial by default the api uses F
    let metric = env::var("METRIC").expect("No metric found using the default");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units={metric}"
    );

    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    match res.status() {
        reqwest::StatusCode::OK => match &res.json::<WeatherResponse>().await {
            Ok(parsed) => print_funny_weather(parsed, &metric),
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

fn print_funny_weather(w: &WeatherResponse, metric: &str) {
    let temp_c = w.main.temp;
    let feels_c = w.main.feels_like;
    let val = match metric {
        "standard" => "F",
        "metric" => "C",
        "imperial" => "IDK",
        other => "F",
    };

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
    println!(
        "║ Temp:        {:>6.1} >{}                   ║",
        temp_c,
        val.to_string()
    );
    println!(
        "║ Feels Like:  {:>6.1}{}                    ║",
        feels_c, val
    );
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
