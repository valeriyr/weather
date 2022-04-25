use chrono::NaiveDate;

use super::settings::Settings;
use super::weather_provider::WeatherData;
use super::weather_provider::WeatherProvider;

use std::io;

pub struct WeatherProviderImpl {
    service_name: &'static str,
    api_key_setting: &'static str,
    url_format: &'static str,
    result_parser: fn(&String) -> Option<WeatherData>,
}

impl WeatherProvider for WeatherProviderImpl {
    fn configure(&self) {
        println!("Provide the API key for the {} provider:", self.service_name);

        let mut api_key = String::new();

        loop {
            match io::stdin().read_line(&mut api_key) {
                Ok(_size) => {
                    let trimmed_api_key = api_key.trim().to_string();

                    println!("Saving the key '{}' to the settings...", &trimmed_api_key);
                    Settings::set(&self.api_key_setting, &trimmed_api_key);

                    break;
                }
                Err(_error) => {
                    println!("Something went wrong, try one more time");
                }
            }
        }
    }

    fn get_weather(&self, city: &String, date: &NaiveDate) -> Option<WeatherData> {
        let mut api_key = Settings::get(&self.api_key_setting);

        if let Some(key) = api_key {
            return self.get_weather_impl(city, date, &key);
        } else {
            println!("Can't find the API key for {} provider...", self.service_name);

            self.configure();

            api_key = Settings::get(&self.api_key_setting);

            if let Some(key) = api_key {
                return self.get_weather_impl(city, date, &key);
            } else {
                println!("Something went wrong...");
                None
            }
        }
    }
}

impl WeatherProviderImpl {
    pub fn new(
        service_name: &'static str,
        api_key_setting: &'static str,
        url_format: &'static str,
        result_parser: fn(&String) -> Option<WeatherData>,
    ) -> WeatherProviderImpl {
        WeatherProviderImpl {
            service_name,
            api_key_setting,
            url_format,
            result_parser,
        }
    }

    fn get_weather_impl(&self, city: &String, _date: &NaiveDate, api_key: &String) -> Option<WeatherData> {
        let url = self.url_format.replace("{city}", city).replace("{api_key}", api_key);

        match reqwest::blocking::get(url) {
            Ok(responce) => match responce.text() {
                Ok(json) => (self.result_parser)(&json),
                Err(error) => {
                    println!("{}", error);
                    None
                }
            },
            Err(error) => {
                println!("{}", error);
                None
            }
        }
    }
}
