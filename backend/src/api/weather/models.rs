use reqwest::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentWeatherResponse {
    pub coord: Coord,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    #[serde(default = "default_1h_rain")]
    #[serde(deserialize_with = "deser_1h_rain")]
    pub rain: RainOneHour,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RainOneHour {
    #[serde(rename = "1h")]
    one_hour: f64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RainThreeHours {
    #[serde(rename = "3h")]
    three_hours: f64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coord  {
    pub lat:f64,
    pub lon:f64
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    pub temp: f64,
    pub grnd_level: i32, // Atmospheric pressure on the ground level, hPa
    pub humidity: i32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    pub id: i32,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastWeatherResponse {
    pub cnt: i32,
    pub list: Vec<ForecastListItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastListItem {
    pub dt: i64,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    #[serde(default = "default_3h_rain")]
    #[serde(deserialize_with = "deser_3h_rain")]
    pub rain: RainThreeHours,
    pub pop: f32, // Probability of precipitation. The values of the parameter vary between 0 and 1, where 0 is equal to 0%, 1 is equal to 100%
}
 
fn default_3h_rain() -> RainThreeHours {
    RainThreeHours{three_hours: 0.0}
}
fn deser_3h_rain<'de, D: Deserializer<'de>>(deserializer: D) -> Result<RainThreeHours, D::Error> {
    Option::deserialize(deserializer).map(|s| s.unwrap_or_else(default_3h_rain))
}

fn default_1h_rain() -> RainOneHour {
    RainOneHour{one_hour: 0.0}
}
fn deser_1h_rain<'de, D: Deserializer<'de>>(deserializer: D) -> Result<RainOneHour, D::Error> {
    Option::deserialize(deserializer).map(|s| s.unwrap_or_else(default_1h_rain))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Clouds {
    all: i32 //
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Wind {
    speed: f32, // Wind speed (meter/sec)
    deg: f32, // Wind direction (degrees)
    gust: f32, // Wind gust (meter/sec)
}

// Errors
#[derive(Debug)]
pub enum FetchCurrentWeatherError {
    RequestError,
    JsonParseError,
    BadStatusError(StatusCode)
}

#[derive(Debug)]
pub enum FetchForecastWeatherError {
    RequestError,
    JsonParseError,
    BadStatusError(StatusCode)
}