use std::io;
use serde::Deserialize;
use colored::*;
use reqwest;

// Struct to deserialize the JSON response from the OpenWeatherMap API.
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description.
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent the main weather parameters.
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u64,
    pressure: u64,
}

// Struct to represent wind information.
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to fetch the weather information from the OpenWeatherMap API.
fn fetch_weather(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let weather_data = response.json::<WeatherResponse>()?;
    Ok(weather_data)
}

// Function to display the weather information.
fn print_weather_info(weather: &WeatherResponse) {
    let description = &weather.weather[0].description;
    let temp = weather.main.temp;
    let humidity = weather.main.humidity;
    let pressure = weather.main.pressure;
    let wind_speed = weather.wind.speed;

    let weather_report = format!(
        "Weather in {}: {}\nTemperature: {:.1}°C\nHumidity: {}%\nPressure: {} hPa\nWind Speed: {:.1} m/s",
        weather.name, description, temp, humidity, pressure, wind_speed
    );

    let colored_report = match description.as_str() {
        "clear sky" => weather_report.yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_report.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "fog" | "sand" | "dust" => weather_report.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_report.bright_cyan(),
        _ => weather_report.normal(),
    };

    println!("{}", colored_report);
}

// Function to get temperature feeling based on the temperature value.
fn get_temperature_feeling(temp: f64) -> &'static str {
    match temp {
        t if t < 0.0 => "freezing",
        t if t < 10.0 => "cold",
        t if t < 20.0 => "cool",
        t if t < 30.0 => "warm",
        _ => "hot",
    }
}

fn main() {
    println!("{}", "Welcome to the Weather Report!".bright_yellow());

    let api_key = "YOUR_API_KEY_HERE"; // Replace with your actual API key

    loop {
        println!("{}", "Enter the city name:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city = city.trim();

        println!("{}", "Enter the country code:".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input!");
        let country_code = country_code.trim();

        match fetch_weather(city, country_code, api_key) {
            Ok(weather) => print_weather_info(&weather),
            Err(e) => eprintln!("Failed to fetch weather data: {}", e),
        }

        println!("{}", "Would you like to check another city? (yes/no):".bright_green());
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input!");
        let response = response.trim().to_lowercase();

        if response != "yes" {
            println!("{}", "Thank you for using the Weather Report!".bright_yellow());
            break;
        }
    }
}

