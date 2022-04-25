pub mod weather_provider;

mod open_weather;
mod settings;
mod weather_api;

use std::str::FromStr;

use weather_provider::WeatherProvider;

use open_weather::OpenWeather;
use settings::Settings;
use weather_api::WeatherAPI;

use chrono::NaiveDate;
//use ipgeolocate::{Locator, Service};
//use futures::executor::block_on;

//use std::str::FromStr;
use strum_macros::EnumString;
use strum_macros::Display;

#[derive(Display, EnumString)]
pub enum Provider
{
    OpenWeather,
    WeatherAPI
}

pub struct Weather {
    provider: Box<dyn WeatherProvider>
}

static PROVIDER_KEY: &str = "PROVIDER";

impl Weather {
    pub fn new() -> Weather {
        let mut provider = Provider::OpenWeather;

        if let Some(s) = Settings::get(PROVIDER_KEY) {
            if let Ok(p) = Provider::from_str(&s) {
                provider = p;
            }
        }

        Weather { provider : Weather::make_provider(provider) }
    }

    pub fn configure(&mut self, provider: Provider) {
        self.provider = Weather::make_provider(provider);
        self.provider.configure();
    }

    pub fn get_weather(&self, city: &String, date: &String) {
        match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                Ok(date) => {
                    match self.provider.get_weather(&city, &date, &String::from("75a6a9a1fa454206804150042222404")) { // 06cf428a8ad911f47b2fb6aa6f94660d
                        Ok(weather) => println!("Weather: {}", weather),
                        Err(error) => println!("Weather Error: {}", error)
                    }
                },
                Err(_error) => {
                    println!("Date can't be parsed. Please, use the format: Year-Month-Day");
                }
            };
    }

    fn make_provider(p: Provider) -> Box<dyn WeatherProvider> {
        match p {
            Provider::OpenWeather => Box::new(OpenWeather{}),
            Provider::WeatherAPI => Box::new(WeatherAPI{})
        }
    }
}
