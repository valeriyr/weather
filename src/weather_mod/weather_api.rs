use chrono::NaiveDate;

use super::weather_provider::WeatherProvider;

pub struct WeatherAPI {
}

impl WeatherProvider for WeatherAPI {
    fn configure(&self) {

    }

    fn get_weather(&self, city: &String, _date: &NaiveDate, api_key: &String) -> Result<String, String> {
        let url = format!(
            "https://api.weatherapi.com/v1/current.json?key={api_key}&q={city}&aqi=no",
            city = city,
            api_key = api_key);

        match reqwest::blocking::get(&url) {
            Ok(responce) => {
                match responce.text() {
                    Ok(result) => Ok(result),
                    Err(error) => {println!("{}", error); Err(String::from("Error1"))}
                }
            },
            Err(_error) => Err(String::from("Error2"))
        }
    }
}