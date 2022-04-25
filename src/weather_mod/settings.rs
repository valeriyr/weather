use config::Config;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub struct Settings {
}

impl Settings {
    pub fn get(key: &str) -> Option<String> {
        if let Ok(settings) = SETTINGS.read() {
            if let Ok(value) = settings.get::<String>(key) {
                return Some(value);
            }
        }

        None
    }

    pub fn set(key: &str, value: String) {
        if let Ok(mut settings) = SETTINGS.write() {
            let _ = settings.set(key, value);
        }
    }
}
