use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub temp_c: f32,
    pub feelslike_c: f32,
    pub pressure_mb: f32,
    pub wind_kph: f32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherAPIResponse {
    pub location: Location,
    pub current: Weather,
}
