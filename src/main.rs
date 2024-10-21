use std::env;
use std::io::{self, Write};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use dotenv::dotenv;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    local_names: HashMap<String, String>,
    lat: f64,
    lon: f64,
    country: String,
    state: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
    sea_level: Option<u32>,  // Optional field (not always present)
    grnd_level: Option<u32>, // Optional field (not always present)
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
    deg: u32,
    gust: Option<f64>, // Optional field (not always present)
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename = "1h")]
    one_hour: Option<f64>, // Optional field for rain in the last hour
    #[serde(rename = "3h")]
    three_hour: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct Clouds {
    all: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Sys {
    r#type: Option<u32>,
    r#id: Option<u32>,
    country: Option<String>,
    sunrise: u64,
    sunset: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WeatherData {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    rain: Option<Rain>, // Optional field (may not always be present)
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    timezone: i32,
    id: u32,
    name: String,
    cod: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WeatherForecast {
    dt: u64,               // Unix timestamp
    main: Main,            // You already have this struct
    weather: Vec<Weather>, // You already have this struct
    clouds: Clouds,        // You already have this struct
    wind: Wind,            // You already have this struct
    visibility: u32,
    pop: f64,           // Probability of precipitation
    rain: Option<Rain>, // You already have this struct, marked as Option because it might not be present
    sys: SysForecast,   // A new struct for sys
    dt_txt: String,     // Date and time in string format
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct City {
    id: i64,
    name: String,
    coord: Coord,
    country: String,
    population: i64,
    timezone: i32,
    sunrise: i64,
    sunset: i64,
}

#[derive(Debug, Deserialize)]
struct Forecast {
    cod: String,
    message: i32,
    cnt: i32,
    list: Vec<WeatherForecast>,
    city: City,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct SysForecast {
    pod: String, // Period of the day ("d" for day, "n" for night)
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Position {
    longitude: f64,
    latitude: f64,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let appid = env::var("API_KEY").expect("API_KEY not set in .env file");

    let country = get_user_input("please enter your country:");

    let city = get_user_input("please enter your city:");

    io::stdout().flush().expect("Failed to flush stdout");
    println!("What would you like to see:");
    println!("1. Hourly forecast (4 days)");
    println!("2. Current weather");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    let choice: i32 = input.trim().parse()?;

    let position = get_long_and_lat(
        appid.clone(),
        country.to_string().clone(),
        city.to_string().clone(),
    );

    match position {
        Ok(position) => {
            if choice == 1 {
                forecast(appid.clone(), position)?;
            } else if choice == 2 {
                current(appid.clone(), position)?;
            } else {
                println!("Invalid choice");
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to fetch position data: {}", e);
            Err(e)
        }
    }
}

fn get_user_input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let input = input.trim();
    input.to_string()
}

fn get_long_and_lat(
    appid: String,
    country: String,
    city: String,
) -> Result<Position, Box<dyn Error>> {
    let geo_url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={},{}&limit=1&appid={}",
        city, country, appid
    );
    let geo_response = reqwest::blocking::get(&geo_url);
    match geo_response {
        Ok(response) => {
            let status = response.status();
            if !status.is_success() {
                eprintln!("Failed to get geo location data: HTTP {}", status);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Geo request failed",
                )));
            }
            let locations: Vec<Location> = response.json()?;
            if locations.is_empty() {
                eprintln!("No location found for the given city and country");
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No locations found",
                )));
            }

            // Get the first location
            let location: &Location = &locations[0];

            let lon = location.lon;
            let lat = location.lat;

            let position = Position {
                longitude: lon,
                latitude: lat,
            };

            Ok(position)
        }
        Err(e) => {
            eprintln!("Failed to fetch geo location data: {}", e);
            Err(Box::new(e))
        }
    }
}

fn pad_string_with_char(input: &str, width: usize, pad_char: char) -> String {
    format!("{:1$}", input, width).replace(' ', &pad_char.to_string())
}

fn forecast(appid: String, position: Position) -> Result<(), Box<dyn Error>> {
    let forecast_data = fetch_forecast_data(appid, position)?;
    let forecast_table = build_forecast_table(forecast_data);
    display_forecast_table(forecast_table);

    Ok(())
}

fn fetch_forecast_data(appid: String, position: Position) -> Result<Forecast, Box<dyn Error>> {
    let forecast_url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&cnt=5&units=metric&appid={}",
        position.latitude, position.longitude, appid
    );

    let forecast_response = reqwest::blocking::get(&forecast_url)?;
    if !forecast_response.status().is_success() {
        eprintln!(
            "Failed to get weather data: HTTP {}",
            forecast_response.status()
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Weather request failed",
        )));
    }

    let forecast_data = forecast_response.json::<Forecast>()?;
    Ok(forecast_data)
}

fn build_forecast_table(forecast_data: Forecast) -> Vec<String> {
    let mut date_string = String::from("date           | ");
    let mut time_string = String::from("time           | ");
    let mut temp_string = String::from("temp range     | ");
    let mut feels_like_string = String::from("feels like     | ");
    let mut humidity_string = String::from("humidity       | ");
    let mut wind_speed_string = String::from("wind speed     | ");
    let mut cloud_coverage_string = String::from("cloud coverage | ");
    let mut rain_string = String::from("rain 3h        | ");

    let divider = "-------------------------------------------------------------------------------------------";

    for forecast in forecast_data.list {
        let mut datetime_parts = forecast.dt_txt.split_whitespace();
        let date = datetime_parts.next().unwrap_or("failed to extract date");
        let time = datetime_parts.next().unwrap_or("failed to extract time");

        // Accumulate the formatted data for each forecast point
        date_string.push_str(&pad_string_with_char(date, 13, ' '));
        date_string.push_str("| ");

        time_string.push_str(&pad_string_with_char(time, 13, ' '));
        time_string.push_str("| ");

        let temp_max = forecast.main.temp_max as i64;
        let temp_min = forecast.main.temp_min as i64;
        temp_string.push_str(&pad_string_with_char(
            &format!("{}°C-{}°C", temp_min, temp_max),
            13,
            ' ',
        ));
        temp_string.push_str("| ");

        let feels_like = forecast.main.feels_like as i64;
        feels_like_string.push_str(&pad_string_with_char(&format!("{}°C", feels_like), 13, ' '));
        feels_like_string.push_str("| ");

        let humidity = forecast.main.humidity;
        humidity_string.push_str(&pad_string_with_char(&format!("{}%", humidity), 13, ' '));
        humidity_string.push_str("| ");

        let wind_speed = forecast.wind.speed as i64;
        wind_speed_string.push_str(&pad_string_with_char(
            &format!("{} m/s", wind_speed),
            13,
            ' ',
        ));
        wind_speed_string.push_str("| ");

        let cloud_coverage = forecast.clouds.all;
        cloud_coverage_string.push_str(&pad_string_with_char(
            &format!("{}%", cloud_coverage),
            13,
            ' ',
        ));
        cloud_coverage_string.push_str("| ");

        let rain = if let Some(rain_data) = &forecast.rain {
            format!("{} mm", rain_data.three_hour.unwrap_or(0.0) as i64)
        } else {
            "None".to_string()
        };
        rain_string.push_str(&pad_string_with_char(&format!("{}", rain), 13, ' '));
        rain_string.push_str("| ");
    }

    vec![
        date_string,
        time_string,
        divider.to_string(),
        temp_string,
        feels_like_string,
        humidity_string,
        wind_speed_string,
        cloud_coverage_string,
        rain_string,
    ]
}

fn display_forecast_table(forecast_table: Vec<String>) {
    for row in forecast_table {
        println!("{}", row);
    }
}

fn fetch_current_data(appid: String, position: Position) -> Result<WeatherData, Box<dyn Error>> {
    let weather_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid={}",
        position.latitude, position.longitude, appid
    );

    let forecast_response = reqwest::blocking::get(&weather_url)?;
    if !forecast_response.status().is_success() {
        eprintln!(
            "Failed to get weather data: HTTP {}",
            forecast_response.status()
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Weather request failed",
        )));
    }

    let weather = forecast_response.json::<WeatherData>()?;
    Ok(weather)
}

fn current(appid: String, position: Position) -> Result<(), Box<dyn Error>> {
    let weather_data = fetch_current_data(appid, position)?;
    display_current_weather(weather_data);

    Ok(())
}

fn display_current_weather(weather: WeatherData) {
    println!("City: {}", weather.name);
    println!(
        "Temperature: {}°C to {}°C",
        weather.main.temp_min, weather.main.temp_max
    );
    println!("Feels like: {}°C", weather.main.feels_like);
    println!("Humidity: {}%", weather.main.humidity);
    println!("Wind speed: {} m/s", weather.wind.speed);
    println!("Cloud coverage: {}%", weather.clouds.all);
    if let Some(rain_data) = &weather.rain {
        println!("Rain in last hour: {}mm", rain_data.one_hour.unwrap_or(0.0));
    } else {
        println!("Rain: None");
    }
}
