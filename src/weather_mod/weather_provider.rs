use chrono::NaiveDate;
use std::fmt;

pub struct WeatherData {
    pub location: String,
    pub temperature: f32,
    pub feelslike: f32,
    pub pressure: f32,
    pub wind: f32,
    pub humidity: i32,
}

impl fmt::Display for WeatherData {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "The temperature in {} is {}, feels like {}, pressure {}, wind {}, humidity {}",
            self.location,
            self.temperature,
            self.feelslike,
            self.pressure,
            self.wind,
            self.humidity
        )
    }
}

pub trait WeatherProvider {
    fn configure(&self);
    fn get_weather(&self, city: &String, date: &NaiveDate) -> Option<WeatherData>;
}
