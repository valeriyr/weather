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
    fn is_configured(&self) -> bool {
        Settings::exists(self.api_key_setting)
    }

    fn configure(&self) {
        println!("Info: Provide the API key for the {} provider:", self.service_name);

        let mut api_key = String::new();

        loop {
            match io::stdin().read_line(&mut api_key) {
                Ok(_size) => {
                    let trimmed_api_key = api_key.trim().to_string();

                    println!("Info: Saving the key '{}' to the settings...", &trimmed_api_key);
                    Settings::set(self.api_key_setting, &trimmed_api_key);

                    break;
                }
                Err(_error) => {
                    println!("Error: Something went wrong. Please, try one more time.");
                }
            }
        }
    }

    fn get_weather(&self, city: &str, _date: &NaiveDate) -> Option<WeatherData> {
        let api_key = Settings::get(self.api_key_setting);

        if let Some(key) = api_key {
            let url = self.url_format.replace("{city}", city).replace("{api_key}", &key);

            match reqwest::blocking::get(url) {
                Ok(responce) => match responce.text() {
                    Ok(json) => {
                        let data = (self.result_parser)(&json);

                        if data.is_none() {
                            println!("Error: The request result from the '{}' provider can't be parsed.", self.service_name);
                        }

                        data
                    },
                    Err(_error) => {
                        println!("Error: Can't get responce from the '{}' provider.", self.service_name);
                        None
                    }
                },
                Err(_error) => {
                    println!("Error: The '{}' provider is unavailable, try again later or configure it.", self.service_name);
                    None
                }
            }
        } else {
            println!("Error: Can't find the API key for '{}' provider.", self.service_name);
            None
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
}
