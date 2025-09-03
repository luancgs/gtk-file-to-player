use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_path: String,
    pub window_title: String,
    pub search_placeholder: Option<String>,
}

impl AppConfig {
    pub fn load() -> Self {
        let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");

        toml::from_str(&config_str).expect("Failed to parse config.toml")
    }
}
