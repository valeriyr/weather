use directories::ProjectDirs;
use lazy_static::lazy_static;
use pickledb::{PickleDb, PickleDbDumpPolicy};
use std::fs;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Option<PickleDb>> = {
        if let Some(proj_dirs) = ProjectDirs::from("com", "reutov", "weather") {
            let config_dir = proj_dirs.config_dir();

            if fs::create_dir_all(config_dir).is_ok() {
                let mut file_path = config_dir.to_path_buf();
                file_path.push("settings.json");

                if let Ok(path) = file_path.into_os_string().into_string() {
                    if let Ok(db) = PickleDb::load_json(&path, PickleDbDumpPolicy::AutoDump) {
                        return RwLock::new(Some(db));
                    } else {
                        return RwLock::new(Some(PickleDb::new_json(&path, PickleDbDumpPolicy::AutoDump)));
                    }
                }
            }
        }

        RwLock::new(None)
    };
}

pub struct Settings {}

impl Settings {
    pub fn exists(key: &str) -> bool {
        if let Ok(lock) = SETTINGS.read() {
            if let Some(db) = lock.as_ref() {
                return db.exists(key);
            }
        }

        false
    }

    pub fn get(key: &str) -> Option<String> {
        if let Ok(lock) = SETTINGS.read() {
            if let Some(db) = lock.as_ref() {
                if let Some(value) = db.get::<String>(key) {
                    return Some(value);
                }
            }
        }

        None
    }

    pub fn set(key: &str, value: &String) {
        if let Ok(mut lock) = SETTINGS.write() {
            if let Some(db) = lock.as_mut() {
                let _ = db.set(key, value);
            }
        }
    }
}
