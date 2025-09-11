use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_path: String,
    pub song_directory: String,
    pub player_socket: String,
    pub window_title: String,
    pub search_placeholder: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, String> {
        let config_str = fs::read_to_string("config.toml")
            .map_err(|e| format!("Failed to read config.toml: {}", e))?;

        toml::from_str(&config_str).map_err(|e| format!("Failed to parse config.toml: {}", e))
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.database_path.is_empty() {
            return Err("Database path cannot be empty".to_string());
        }

        if self.song_directory.is_empty() {
            return Err("Song directory cannot be empty".to_string());
        }

        if self.player_socket.is_empty() {
            return Err("Player socket cannot be empty".to_string());
        }

        if self.window_title.is_empty() {
            return Err("Window title cannot be empty".to_string());
        }

        if !std::path::Path::new(&self.song_directory).exists() {
            return Err(format!(
                "Song directory does not exist: {}",
                self.song_directory
            ));
        }

        Ok(())
    }
}
