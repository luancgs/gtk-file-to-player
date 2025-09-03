use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_path: String,
    pub song_directory: String,
    pub window_title: String,
    pub search_placeholder: Option<String>,
}

impl AppConfig {
    pub fn load() -> Self {
        let config_str = fs::read_to_string("config.toml");

        match config_str {
            Ok(config_str) => match toml::from_str(&config_str) {
                Ok(config) => config,
                Err(err) => {
                    eprintln!("Failed to parse config.toml: {}", err);
                    std::process::exit(1);
                }
            },
            Err(err) => {
                eprintln!("Failed to read config.toml: {}", err);
                std::process::exit(1);
            }
        }
    }
}
