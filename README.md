# Rust Weather CLI

A small command-line weather app written in Rust. It uses the Open-Meteo geocoding and forecast APIs to fetch current weather for a city.

## Features

- Search current weather by city name
- Save cities for later
- List saved cities
- Use a saved city by number
- Delete saved cities by number
- Choose between funny and compact weather displays

## Usage

```text
cargo run -- <city>           Get weather for a city
cargo run -- <number>         Get weather for a saved city by number
cargo run -- ls               List saved cities
cargo run -- delete <number>  Delete a saved city
cargo run -- <city> --save    Save city without prompting
cargo run -- <city> --no-save Skip save prompt
cargo run -- <city> --display compact
cargo run -- --help           Show help
```

### Examples

Get weather for a city:

```bash
cargo run -- Athens
```

Example output:

```text
╔════════════════════════════════════════════╗
║        🧙 WEATHER GOBLIN REPORT 🧙        ║
╠════════════════════════════════════════════╣
║ City:        Athens                       ║
║ Sky Drama:   partly cloudy                ║
║ Temp:         22.5°C                      ║
║ Feels Like:   20.1°C                      ║
║ Humidity:       65%                        ║
║ Wind:         12.3km/h                    ║
╠════════════════════════════════════════════╣
║ 😎 Pretty nice. The sun is behaving... for now.        ║
║ Wind status: gentle breeze doing side quests           ║
╚════════════════════════════════════════════╝
```

Save a city without the interactive prompt:

```bash
cargo run -- Copenhagen --save
```

Skip the save prompt:

```bash
cargo run -- London --no-save
```

Use the compact display:

```bash
cargo run -- Athens --display compact
```

```text
Athens: 22.5°C, feels 20.1°C, partly cloudy, humidity 65%, wind 12.3km/h
```

List saved cities:

```bash
cargo run -- ls
```

```text
Saved cities:
1. Athens
2. Copenhagen
```

Use a saved city by number:

```bash
cargo run -- 1
```

Delete a saved city by number:

```bash
cargo run -- delete 1
cargo run -- del 1
cargo run -- rm 1
```

## Saved Cities

Saved cities are stored in `cities.json` in this format:

```json
{
  "cities": [
    "Athens",
    "Copenhagen"
  ]
}
```

Older files with this format are also supported:

```json
{
  "city": "Athens"
}
```

The next time a city is saved, the file is rewritten using the newer `cities` list format.

## Development

Format the code:

```bash
cargo fmt
```

Check the project:

```bash
cargo check
```

Run the app:

```bash
cargo run -- Athens
```

Run tests:

```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are accepted. Feel free to open an issue or submit a pull request.
