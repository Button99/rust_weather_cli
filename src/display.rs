use crate::model::WeatherResponse;

pub fn print_funny_weather(w: &WeatherResponse, city: &str) {
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
        temp_c, w.current_units.temperature_2m
    );
    println!(
        "║ Feels Like:  {:>6.1}{}                    ║",
        feels_c, w.current_units.apparent_temperature
    );
    println!(
        "║ Humidity:    {:>6.0}{}                      ║",
        w.current.relative_humidity_2m, w.current_units.relative_humidity_2m
    );
    println!(
        "║ Wind:        {:>6.1}{}                  ║",
        w.current.wind_speed_10m, w.current_units.wind_speed_10m
    );
    println!("╠════════════════════════════════════════════╣");
    println!("║ {:<42} ║", mood);
    println!("║ Wind status: {:<29} ║", wind_comment);
    println!("╚════════════════════════════════════════════╝");
    println!();
}

pub fn weather_description(code: u8) -> &'static str {
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
