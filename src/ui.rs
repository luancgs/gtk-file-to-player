use crate::{config::AppConfig, database::Song, player::Player};
use gtk::gio::Notification;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ButtonsType, Label, MessageDialog, MessageType};

pub struct ErrorDialog;

impl ErrorDialog {
    pub fn show_and_exit(app: &Application, message: &str) {
        let dialog = MessageDialog::builder()
            .message_type(MessageType::Error)
            .buttons(ButtonsType::Ok)
            .title("Error")
            .text("Application Error")
            .secondary_text(message)
            .modal(true)
            .build();

        // Create a dummy window as parent for the dialog
        let window = ApplicationWindow::builder()
            .application(app)
            .visible(false)
            .build();

        dialog.set_transient_for(Some(&window));

        dialog.connect_response(move |dialog, _| {
            dialog.close();
            std::process::exit(1);
        });

        window.present();
        dialog.present();
    }
}

pub struct NotificationSystem;

impl NotificationSystem {
    pub fn send_error(app: &Application, message: &str) {
        let notification = Notification::new("Error");
        notification.set_body(Some(message));
        app.send_notification(None, &notification);
    }

    pub fn send_now_playing(app: &Application, song_title: &str) {
        let notification = Notification::new("Now Playing");
        notification.set_body(Some(song_title));
        app.send_notification(Some("now_playing"), &notification);
    }
}

pub struct SongButton {
    button: Button,
}

impl SongButton {
    pub fn new(config: &AppConfig, song: &Song, app_weak: gtk::glib::WeakRef<Application>) -> Self {
        let button = Button::new();
        let text = format!("{}. {}", song.number, song.title);
        let label = Label::builder()
            .label(&text)
            .halign(gtk::Align::Start)
            .build();
        button.set_child(Some(&label));

        let song_full_path = format!("{}/{}", config.song_directory, song.file);
        let song_title = format!("{}. {}", song.number, song.title);
        let player = Player::new();

        button.connect_clicked(move |_| match player.play_file(&song_full_path) {
            Ok(_) => {
                println!("Playing: {}", song_title);
                if let Some(app) = app_weak.upgrade() {
                    NotificationSystem::send_now_playing(&app, &song_title);
                }
            }
            Err(e) => {
                eprintln!("Failed to play song: {}", e);
                if let Some(app) = app_weak.upgrade() {
                    NotificationSystem::send_error(&app, &format!("Failed to play: {}", e));
                }
            }
        });

        SongButton { button }
    }

    pub fn widget(&self) -> &Button {
        &self.button
    }
}
