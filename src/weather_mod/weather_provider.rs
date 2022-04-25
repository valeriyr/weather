use chrono::NaiveDate;

pub trait WeatherProvider
{
    fn configure(&self);
    fn get_weather(&self, city: &String, date: &NaiveDate, api_key: &String) -> Result<String, String>;
}
