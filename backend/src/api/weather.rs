use std::env;
pub mod models;


pub async fn fetch_current(lat:f64, lon:f64) -> Result<models::CurrentWeatherResponse, models::FetchCurrentWeatherError>{
    let api_key: String = env::var("OPENWEATHERMAP_API_KEY").expect(".env doesnt have api key");

    // Build the URL for the OpenWeatherMap API request
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={api_key}");

     let response = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            return Err(models::FetchCurrentWeatherError::RequestError);
        }
     };
     if response.status().is_success() {
        // Parse the JSON response into our WeatherData struct
        let weather_data: models::CurrentWeatherResponse = match response.json().await {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return Err(models::FetchCurrentWeatherError::JsonParseError)
            }
        };
        // Extract and print relevant weather information
        let temperature = weather_data.main.temp;
        let description = &weather_data.weather[0].description;
        println!("Weather: {:.2}K, {}", temperature, description);
        Ok(weather_data)
    } else {
        // Print an error message if the request was not successful
        let status = response.status();
        println!("Error: {}", status);
        Err(models::FetchCurrentWeatherError::BadStatusError(status) )
    }
}

pub async fn fetch_forecast(lat:f64, lon:f64) -> Result<models::ForecastWeatherResponse, models::FetchForecastWeatherError>{
    let api_key: String = env::var("OPENWEATHERMAP_API_KEY").expect(".env doesnt have api key");

    // Build the URL for the OpenWeatherMap API request
    let url = format!("https://api.openweathermap.org/data/2.5/forecast?lat={lat}&lon={lon}&appid={api_key}");

     let response = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            return Err(models::FetchForecastWeatherError::RequestError);
        }
     };
     if response.status().is_success() {
        println!("{:?}",response);
        // Parse the JSON response into our WeatherData struct
        let weather_data: models::ForecastWeatherResponse = match response.json().await {
            Ok(r) => r,
            Err(e) => {
                println!("Error parsing JSON: {}", e);
                return Err(models::FetchForecastWeatherError::JsonParseError)
            }
        };
        // Extract and print relevant weather information
        let count = weather_data.cnt;
        let list = &weather_data.list;
        println!("Forecast {count}: {:?}", list);
        Ok(weather_data)
    } else {
        // Print an error message if the request was not successful
        let status = response.status();
        println!("Error: {}", status);
        Err(models::FetchForecastWeatherError::BadStatusError(status) )
    }
}