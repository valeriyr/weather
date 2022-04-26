pub mod weather_provider;

mod open_weather;
mod weather_api;
mod settings;
mod weather_provider_impl;

use open_weather::OpenWeatherResponse;
use weather_api::WeatherAPIResponse;
use settings::Settings;
use weather_provider::WeatherData;
use weather_provider::WeatherProvider;
use weather_provider_impl::WeatherProviderImpl;

use chrono::{Local, NaiveDate};

use strum_macros::Display;
use strum_macros::EnumString;

use std::str::FromStr;

#[derive(Display, Clone, Copy, EnumString)]
pub enum Provider {
    OpenWeather,
    WeatherAPI,
}

pub struct Weather {
    provider: Box<dyn WeatherProvider>,
}

static PROVIDER_KEY: &str = "PROVIDER_KEY";
static DEFAULT_PROVIDER: Provider = Provider::OpenWeather;

pub static CURRENT_DATE_NAME: &str = "NOW";

impl Weather {
    pub fn new() -> Weather {
        println!("Info: Initialization...");

        let mut provider = None;

        if let Some(s) = Settings::get(PROVIDER_KEY) {
            if let Ok(p) = Provider::from_str(&s) {
                println!("Info: The provider '{p}' was restored from the settings.");
                provider = Some(p);
            }
        }

        if provider.is_none() {
            provider = Some(DEFAULT_PROVIDER);
            println!("Info: The provider can't be restored from the settings. Using the default one '{DEFAULT_PROVIDER}'.");

            Settings::set(PROVIDER_KEY, &DEFAULT_PROVIDER.to_string());
        }

        Weather{ provider: Weather::make_provider(provider.unwrap()) }
    }

    pub fn configure(&mut self, provider: Provider) {
        println!("Info: Configuration...");
        println!("Info: Saving the provider '{provider}' to the settings...");

        Settings::set(PROVIDER_KEY, &provider.to_string());

        self.provider = Weather::make_provider(provider);
        self.provider.configure();
    }

    pub fn get_weather(&self, city: &str, date: &String) {
        if !self.provider.is_configured() {
            println!("Info: The provider need to be configured.");
            self.provider.configure();
        }

        println!("Info: Getting data...");

        let final_date;

        if date == CURRENT_DATE_NAME {
            final_date = Local::now().naive_local().date();
        }
        else {
            match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                Ok(date) => final_date = date,
                Err(_error) => {
                    println!("Error: The date can't be parsed. Please, use the format: Year-Month-Day.");
                    return;
                }
            };
        }

        match self.provider.get_weather(city, &final_date) {
            Some(data) => println!("{}", data),
            None => (),
        }
    }

    fn make_provider(provider: Provider) -> Box<dyn WeatherProvider> {
        match provider {
            Provider::OpenWeather => Box::new(WeatherProviderImpl::new(
                "OpenWeather",
                "OPEN_WEATHER_API_KEY",
                "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}",
                |json: &String| -> Option<WeatherData> {
                    if let Ok(data) = serde_json::from_str::<OpenWeatherResponse>(json) {
                        return Some(WeatherData{
                            location: data.name,
                            temperature: data.main.temp,
                            feelslike: data.main.feels_like,
                            pressure: data.main.pressure,
                            wind: data.wind.speed,
                            humidity: data.main.humidity});
                    }
                    None
                },
            )),
            Provider::WeatherAPI => Box::new(WeatherProviderImpl::new(
                "WeatherAPI",
                "WEATHER_API_API_KEY",
                "https://api.weatherapi.com/v1/current.json?key={api_key}&q={city}&aqi=no",
                |json: &String| -> Option<WeatherData> {
                    if let Ok(data) = serde_json::from_str::<WeatherAPIResponse>(json) {
                        return Some(WeatherData{
                            location: data.location.name,
                            temperature: data.current.temp_c,
                            feelslike: data.current.feelslike_c,
                            pressure: data.current.pressure_mb,
                            wind: data.current.wind_kph,
                            humidity: data.current.humidity});
                    }
                    None
                },
            )),
        }
    }
}
