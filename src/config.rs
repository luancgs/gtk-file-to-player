use crate::ui;
use gtk::Application;
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
    pub fn load(app: &Application) -> Option<Self> {
        match fs::read_to_string("config.toml") {
            Ok(config_str) => match toml::from_str(&config_str) {
                Ok(config) => config,
                Err(err) => {
                    ui::show_error_and_exit(app, &format!("Failed to parse config.toml: {}", err));
                    None
                }
            },
            Err(err) => {
                ui::show_error_and_exit(app, &format!("Failed to read config.toml: {}", err));
                None
            }
        }
    }
}
