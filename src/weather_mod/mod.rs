pub mod weather_provider;

mod settings;
mod weather_provider_impl;

use settings::Settings;
use weather_provider::WeatherData;
use weather_provider::WeatherProvider;
use weather_provider_impl::WeatherProviderImpl;

use chrono::NaiveDate;

use strum_macros::Display;
use strum_macros::EnumString;

use std::str::FromStr;

#[derive(Display, EnumString)]
pub enum Provider {
    OpenWeather,
    WeatherAPI,
}

pub struct Weather {
    provider: Box<dyn WeatherProvider>,
}

static PROVIDER_KEY: &str = "PROVIDER_KEY";

impl Weather {
    pub fn new() -> Weather {
        let mut provider = None;

        if let Some(s) = Settings::get(PROVIDER_KEY) {
            if let Ok(p) = Provider::from_str(&s) {
                println!("The provider '{}' was restored from the settings", p);
                provider = Some(p);
            }
        }

        if let None = provider {
            provider = Some(Provider::OpenWeather);
            println!(
                "The provider can't be restored from the settings. Using the default one '{}'",
                provider.as_ref().unwrap()
            );
        }

        Weather {
            provider: Weather::make_provider(provider.unwrap()),
        }
    }

    pub fn configure(&mut self, provider: Provider) {
        println!("Configuration...");
        println!(
            "Saving provider '{}' to the settings...",
            provider.to_string()
        );

        Settings::set(PROVIDER_KEY, &provider.to_string());

        self.provider = Weather::make_provider(provider);
        self.provider.configure();
    }

    pub fn get_weather(&self, city: &String, date: &String) {
        match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
            Ok(date) => match self.provider.get_weather(&city, &date) {
                Some(data) => Weather::print_weather_data(&data),
                None => (),
            },
            Err(_error) => {
                println!("The date can't be parsed. Please, use the format: Year-Month-Day");
            }
        };
    }

    fn make_provider(p: Provider) -> Box<dyn WeatherProvider> {
        match p {
            Provider::OpenWeather => Box::new(WeatherProviderImpl::new(
                "Open Weather",
                "OPEN_WEATHER_API_KEY",
                "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}",
                |_data: &String| -> Option<WeatherData> { Some(WeatherData{ temperature: String::from("1") }) },
            )),
            Provider::WeatherAPI => Box::new(WeatherProviderImpl::new(
                "Weather API",
                "WEATHER_API_API_KEY",
                "https://api.weatherapi.com/v1/current.json?key={api_key}&q={city}&aqi=no",
                |_data: &String| -> Option<WeatherData> { Some(WeatherData{ temperature: String::from("2") })
                },
            )),
        }
    }

    fn print_weather_data(data: &WeatherData) {
        println!("Temperature: {}", data.temperature);
    }
}
