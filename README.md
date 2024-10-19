# Weather CLI App

A command-line application to fetch current weather information and future forecasts for any city using the [OpenWeatherMap API](https://openweathermap.org/).

## Features

- **Current Weather**: Retrieve real-time weather data for a specified city and country.
- **Hourly Forecast**: *(Coming Soon)* Get a 4-day hourly weather forecast for a specified location.

## Requirements

- **Rust**: Ensure you have Rust installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).
- **OpenWeatherMap API Key**: Sign up for a free API key at [OpenWeatherMap](https://home.openweathermap.org/users/sign_up).

## Installation

1. **Clone the Repository**

   ```
   git clone https://github.com/yourusername/weather-cli-app.git
   cd weather-cli-app
   ```

2. **Set Up Environment Variables**

   Create a `.env` file in the root directory of the project and add your OpenWeatherMap API key:

   ```
   touch .env
   ```

   Open the `.env` file in your favorite text editor and add:

   ```
   API_KEY=your_openweathermap_api_key
   ```

3. **Install Dependencies**

   Ensure your `Cargo.toml` includes the necessary dependencies:

   ```
   [dependencies]
   reqwest = { version = "0.11", features = ["blocking", "json"] }
   serde = { version = "1.0", features = ["derive"] }
   dotenv = "0.15"
   ```

## Compilation

1. **Build the Application**

   To compile the application, run:

   ```
   cargo build --release
   ```

   The compiled binary will be located in the `target/release/` directory.

2. **Run the Application**

   You can run the application using Cargo:

   ```
   cargo run
   ```

   Alternatively, run the compiled binary directly:

   ```
   ./target/release/weather-cli-app
   ```

## Usage

1. **Run the Application**

   Execute the application using Cargo or the compiled binary.

2. **Enter Country and City**

   The app will prompt you to enter the country and city for which you want to retrieve weather data.
   ```
   please enter your country: United states 
   please enter your city: New York
   ```

3. **Choose an Option**

   Select the type of weather information you want:
   ```
   What would you like to see:
    1. Hourly forecast (4 days)
    2. Current weather
   ```

Enter `1` for the hourly forecast or `2` for the current weather.

4. **View Results**

Based on your selection, the app will display the requested weather information.

**Example Output for Current Weather:**
```
City: New York 
Temperature: 15°C to 20°C 
Feels like: 18°C 
Humidity: 60% 
Wind speed: 5 m/s 
Cloud coverage: 75% 
Rain: None
```

## API Integration

This application uses the [OpenWeatherMap API](https://openweathermap.org/api) to fetch weather data.

### Adding Your API Key

1. **Obtain an API Key**

Sign up at [OpenWeatherMap](https://home.openweathermap.org/users/sign_up) to get your free API key.

2. **Set the API Key in `.env`**

Ensure your `.env` file contains your API key:

```
API_KEY=your_openweathermap_api_key
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


