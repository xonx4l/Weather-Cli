use std::io;
use serde::Deserialize;
use colored::*;

// struct to deserialise the JSON response from openWeatherMap API.
#[derive(Deserialize, Debug)]

struct WeatherResponse {
     weather : Vec<Weather>,
     main: Main,
     wind: Wind,
     name: String,
}

// struct to represent weather description 
#[derive(Deserialize , Debug )]

struct Weather {
    description: String,
} 

//struct to represent the main weather parameters 
#[derive (Deserialize, Debug)]

struct Main {
     temp: f64,
     humidity: f64,
     pressure: f64
}

#[derive(Deserialize , Debug)]

struct Wind {
   speed: f64,
}

// Function to get the weather information from OpenWeatherMap API.
 fn get_weather_info(city:&str, country_code: &str, api_key: &str ) ->
 Result<WeatherResponse, reqwest::Error>{
      let url: String = format!(
          "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={}", city, country_code, api_key
      );
  let response = reqwest::blocking::get(&url)?;
  let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
   Ok(response_json);
 }

 // Function to display the weather information 
  fn display_weather_info(response: &WeatherResponse) {
        // Extract the weather information from the response 
        let description : &String = &response.weather[0].description;
        let temperature : &String = &response.main.temp;
        let humidity : &String = &response.main.humidity;
        let pressure : &String = &response.main.pressure;
        let wind_speed : &String = &response.wind.speed;
        // Formatting weather information inta a string 
        let weather_text: String = format!(
        "Weather in {}: {} {}"
        > Temperature: {:.1}C,
        > Humidity: {:.1}%,
        > Pressure: {:.1}hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_feeling(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed
        );

      // Coloring the weather text based on weather conditions 
         let weather_text_colored:coloredString = match description.as_str(){
              "clear sky" => weather_text.yellow(),
               "few clouds" | "scattered clouds" | "broken clouds" =>
               weather_text.bright_blue(),
               "overcast clouds" | "mist" | "haze" | "smoke" | "sand" |
               "dust"| "fog" |"squalls" => weather_text.dimmed(),
               "shower rain"| |"rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
              _=> weather_text.normal(),


        // function to get feeling based on temperature 
         fn get_temp_feeling(temperature:f64) -> &'static str {
             if temperature <0.0 {
             "freeze"
             } else if temperature >= 0.0 && temperature < 10.0{
             "cloudy"
             } else if temperature >= 20.0 && temperature <30.0{
             "sunny"
             }else {
              "hot"
             }
         }


  }
