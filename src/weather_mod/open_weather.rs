use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Temperature {
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: f32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
}

#[derive(Deserialize, Debug)]
pub struct OpenWeatherResponse {
    pub name: String,
    pub wind: Wind,
    pub main: Temperature,
}
