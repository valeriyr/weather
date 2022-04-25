use chrono::NaiveDate;

pub struct WeatherData {
    pub temperature: String,
}

pub trait WeatherProvider {
    fn configure(&self);
    fn get_weather(&self, city: &String, date: &NaiveDate) -> Option<WeatherData>;
}
