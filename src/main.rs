mod app;
mod config;
mod database;
mod player;
mod ui;

use config::AppConfig;
use database::Database;
use gtk::prelude::*;
use gtk::{Application, glib};
use player::Player;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.luancgs.file-to-player";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|gtk_app| {
        // Step 1: Load configuration
        let config = match AppConfig::load() {
            Ok(config) => match config.validate() {
                Ok(_) => config,
                Err(e) => {
                    ui::ErrorDialog::show_and_exit(gtk_app, &format!("Configuration Error: {}", e));
                    return;
                }
            },
            Err(e) => {
                ui::ErrorDialog::show_and_exit(gtk_app, &format!("Configuration Error: {}", e));
                return;
            }
        };

        // Step 2: Ensure VLC is available
        if let Err(e) = Player::ensure_available() {
            ui::ErrorDialog::show_and_exit(gtk_app, &format!("Media Player Error: {}", e));
            return;
        }

        // Step 3: Connect to database
        let database = match Database::new(&config.database_path) {
            Ok(db) => Rc::new(RefCell::new(db)),
            Err(e) => {
                ui::ErrorDialog::show_and_exit(gtk_app, &format!("Database Error: {}", e));
                return;
            }
        };

        // Step 4: Build and present the main UI
        match app::MainWindow::new(gtk_app, &config, database) {
            Ok(window) => {
                window.present();
            }
            Err(e) => {
                ui::ErrorDialog::show_and_exit(gtk_app, &format!("UI Error: {}", e));
            }
        }
    });

    app.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_id_is_valid() {
        assert!(APP_ID.contains('.'));
        assert!(APP_ID.len() > 5);
    }

    #[test]
    fn test_app_id_format() {
        let parts: Vec<&str> = APP_ID.split('.').collect();
        assert!(parts.len() >= 3);
        assert!(!parts.iter().any(|&part| part.is_empty()));
    }
}
