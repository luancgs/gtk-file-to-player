use std::path::Path;
use std::process::{Child, Command};

pub struct Player;

impl Player {
    pub fn new() -> Self {
        Player
    }

    pub fn ensure_available() -> Result<(), String> {
        let output = Command::new("vlc")
            .arg("--version")
            .output()
            .map_err(|e| format!("Failed to check VLC: {}", e))?;

        if !output.status.success() {
            return Err("VLC is not installed or not available in PATH".to_string());
        }

        Ok(())
    }

    pub fn play_file(&self, file_path: &str) -> Result<Child, String> {
        if !Path::new(file_path).exists() {
            return Err(format!("File does not exist: {}", file_path));
        }

        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        if !Self::is_supported_format(&extension) {
            return Err(format!("Unsupported file format: .{}", extension));
        }

        Command::new("vlc")
            .arg("--play-and-exit")
            .arg(file_path)
            .spawn()
            .map_err(|e| format!("Failed to start VLC with GUI: {}", e))
    }

    fn is_supported_format(extension: &str) -> bool {
        matches!(
            extension,
            "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" | "wma" | "opus" | "mp4" | "avi" | "mkv"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_formats() {
        assert!(Player::is_supported_format("mp3"));
        assert!(Player::is_supported_format("mp4"));
        assert!(!Player::is_supported_format("txt"));
        assert!(!Player::is_supported_format(""));
    }
}
