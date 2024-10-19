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


pub fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let appid = env::var("API_KEY").expect("API_KEY not set in .env file");

    print!("please enter your country:");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut country = String::new();
    io::stdin().read_line(&mut country).expect("failed to read line");
    let country = country.trim();

    print!("please enter your city:");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("failed to read line");
    let city = city.trim();

    io::stdout().flush().expect("Failed to flush stdout");

    println!("What would you like to see:");
    println!("1. Hourly forecast (4 days)");
    println!("2. Current weather");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    let choice: i32 = input.trim().parse()?;

    if choice == 1 {
        forecast(appid.clone(), country.to_string().clone(), city.to_string().clone())?;
    } else if choice == 2 {
        current(appid.clone(), country.to_string().clone(), city.to_string().clone())?;
    } else {
        println!("Invalid choice");
    }

    Ok(())
}

#[allow(unused_variables)]
fn forecast(appid: String, country: String, city: String) ->Result<(), Box<dyn Error>> {
    println!("you requested a forecast");
    Ok(())
}

fn current(appid: String, country: String, city: String) -> Result<(), Box<dyn Error>> {
    let geo_url = format!("http://api.openweathermap.org/geo/1.0/direct?q={},{}&limit=1&appid={}", city, country, appid);
    let geo_response = reqwest::blocking::get(&geo_url);
    Ok(match geo_response {
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

            // Get weather data
            let weather_url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid={}", lat, lon, appid);
    
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
                            println!("City: {}", location.name);
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
        },
        Err(e) => {
            eprintln!("Failed to fetch geo location data: {}", e);
            return Err(Box::new(e));
        }
    })
    }
