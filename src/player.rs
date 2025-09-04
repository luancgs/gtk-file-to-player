use gtk::Application;

use crate::ui;
use std::process::Command;

pub fn ensure_player(app: &Application) -> Option<()> {
    let status = Command::new("vlc").arg("--version").output();

    match status {
        Ok(output) => {
            if output.status.success() {
                Some(())
            } else {
                ui::show_error_and_exit(app, &format!("VLC is not installed or not available"));
                None
            }
        }

        Err(e) => {
            ui::show_error_and_exit(app, &format!("Failed to check VLC status: {}", e));
            None
        }
    }
}

pub fn play_song(song_file: &str) -> Option<()> {
    let status = Command::new("vlc").arg(song_file).spawn();

    match status {
        Ok(child) => {
            println!("VLC process started: {:?}", child.id());
            Some(())
        }
        Err(e) => {
            eprintln!("Failed to start VLC process: {}", e);
            None
        }
    }
}
