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
    gust: Option<f64>,  // Optional field (not always present)
}

#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename = "1h")]
    one_hour: Option<f64>,  // Optional field for rain in the last hour
    #[serde(rename = "3h")]
    three_hour: Option<f64>
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
    rain: Option<Rain>,   // Optional field (may not always be present)
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    timezone: i32,
    id: u32,
    name: String,
    cod: u32,
}

#[derive(Debug, Deserialize)]
struct WeatherForecast {
    dt: u64,                   // Unix timestamp
    main: Main,                 // You already have this struct
    weather: Vec<Weather>,      // You already have this struct
    clouds: Clouds,             // You already have this struct
    wind: Wind,                 // You already have this struct
    visibility: u32,
    pop: f64,                   // Probability of precipitation
    rain: Option<Rain>,         // You already have this struct, marked as Option because it might not be present
    sys: SysForecast,           // A new struct for sys
    dt_txt: String,             // Date and time in string format
}

#[derive(Debug, Deserialize)]
struct City {
    id: i64,
    name: String,
    coord: Coord,
    country: String,
    population: i64,
    timezone: i32,
    sunrise: i64,
    sunset: i64
}

#[derive(Debug, Deserialize)]
struct Forecast {
    cod: String,
    message: i32,
    cnt: i32,
    list: Vec<WeatherForecast>,
    city: City
}

#[derive(Debug, Deserialize)]
struct SysForecast {
    pod: String,                // Period of the day ("d" for day, "n" for night)
}

#[derive(Debug, Deserialize)]
struct Position {
    longitude: f64,
    latitude: f64
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

    let position = get_long_and_lat(appid.clone(), country.to_string().clone(), city.to_string().clone());

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
    print!("{}",text);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let input = input.trim();
    input.to_string()
}

fn get_long_and_lat(appid: String, country: String, city: String) -> Result<Position, Box<dyn Error>> {
    let geo_url = format!("http://api.openweathermap.org/geo/1.0/direct?q={},{}&limit=1&appid={}", city, country, appid);
    let geo_response = reqwest::blocking::get(&geo_url);
    match geo_response {
        Ok(response) => {
            let status = response.status();
            if !status.is_success() {
                eprintln!("Failed to get geo location data: HTTP {}", status);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Geo request failed")));
            }
            let locations: Vec<Location> = response.json()?;
            if locations.is_empty() {
                eprintln!("No location found for the given city and country");
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "No locations found")));
            }
    
            // Get the first location
            let location: &Location = &locations[0];
    
            let lon = location.lon;
            let lat = location.lat;

            let position = Position {
                longitude: lon,
                latitude: lat
            };

            Ok(position)
        },
        Err(e) => {
            eprintln!("Failed to fetch geo location data: {}", e);
            Err(Box::new(e))
        }
    }
}

#[allow(unused_variables)]
fn forecast(appid: String, position: Position) ->Result<(), Box<dyn Error>> {

    // Get weather data
    let forecast_url = format!("https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&units=metric&appid={}", position.latitude, position.longitude, appid);

    // Make the weather request and print raw response for debugging
    let forecast_response = reqwest::blocking::get(&forecast_url);

    match forecast_response {
        Ok(response) => {
            let status = response.status();
            if !status.is_success() {
                eprintln!("Failed to get weather data: HTTP {}", status);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Weather request failed")));
            }
            match response.json::<Forecast>() {
                Ok(forecast_data) =>  {
                    for forecast in forecast_data.list {
                        println!("Forecast Time: {}", forecast.dt_txt);
                        println!("Temperature: {}°C to {}°C", forecast.main.temp_min, forecast.main.temp_max);
                        println!("Feels like: {}°C", forecast.main.feels_like);
                        println!("Humidity: {}%", forecast.main.humidity);
                        println!("Wind speed: {} m/s", forecast.wind.speed);
                        println!("Cloud coverage: {}%", forecast.clouds.all);
                
                        if let Some(rain_data) = &forecast.rain {
                            println!("Rain in last hour: {}mm", rain_data.three_hour.unwrap_or(0.0));
                        } else {
                            println!("Rain: None");
                        }
                
                        println!("---");
                    };
                    
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to fetch weather data: {}", e);
                    return Err(Box::new(e));
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch weather data: {}", e);
            return Err(Box::new(e));
        }
    }
}

fn current(appid: String, position: Position) -> Result<(), Box<dyn Error>> {

    // Get weather data
    let weather_url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid={}", position.latitude, position.longitude, appid);

    // Make the weather request and print raw response for debugging
    let weather_response = reqwest::blocking::get(&weather_url);

    match weather_response {
        Ok(response) => {
            let status = response.status();
            if !status.is_success() {
                eprintln!("Failed to get weather data: HTTP {}", status);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Weather request failed")));
            }
            match response.json::<WeatherData>() {
                Ok(weather) =>  {
                    println!("City: {}", weather.name);
                    println!("Temperature: {}°C to {}°C", weather.main.temp_min, weather.main.temp_max);
                    println!("Feels like: {}°C", weather.main.feels_like);
                    println!("Humidity: {}%", weather.main.humidity);
                    println!("Wind speed: {} m/s", weather.wind.speed);
                    println!("Cloud coverage: {}%", weather.clouds.all);
                    if let Some(rain_data) = &weather.rain {
                        println!("Rain in last hour: {}mm", rain_data.one_hour.unwrap_or(0.0));
                    } else {
                        println!("Rain: None");
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to fetch weather data: {}", e);
                    return Err(Box::new(e));
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch weather data: {}", e);
            return Err(Box::new(e));
        }
    }
       
}
