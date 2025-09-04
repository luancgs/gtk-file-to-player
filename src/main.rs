mod config;
mod database;
mod player;
mod ui;

use config::AppConfig;
use database::Song;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, Entry, Label, ListBox, Orientation, ScrolledWindow,
    glib,
};
use sqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.luancgs.file-to-player";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let config = match AppConfig::load(app) {
            Some(config) => config,
            None => return,
        };

        player::ensure_player(app);

        let connection = Rc::new(database::connect_to_database(&config.database_path));

        build_ui(app, config, connection);
    });

    app.run()
}

fn build_ui(app: &Application, config: AppConfig, connection: Rc<Connection>) {
    let list_box = Rc::new(RefCell::new(ListBox::new()));
    list_box
        .borrow_mut()
        .set_selection_mode(gtk::SelectionMode::None);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(400)
        .child(&*list_box.borrow())
        .build();

    let search_entry = Entry::builder()
        .placeholder_text(config.search_placeholder.as_deref().unwrap_or("Search..."))
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .primary_icon_name("system-search")
        .build();

    let connection_clone = Rc::clone(&connection);
    let list_box_clone = Rc::clone(&list_box);
    let config_clone = config.clone();

    search_entry.connect_changed(move |entry| {
        let text = entry.text().to_lowercase();

        while let Some(child) = list_box_clone.borrow().first_child() {
            list_box_clone.borrow().remove(&child);
        }

        let results = database::get_songs(&connection_clone, &text).unwrap();

        for song in results.iter() {
            let button = create_song_button(config_clone.clone(), song);
            list_box_clone.borrow().append(&button);
        }
    });

    let main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(6)
        .build();

    main_box.append(&search_entry);
    main_box.append(&scrolled_window);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(config.window_title)
        .default_width(400)
        .default_height(600)
        .child(&main_box)
        .build();

    window.present();
}

fn create_song_button(config: AppConfig, song: &Song) -> Button {
    let button = Button::new();
    let text = format!("{}. {}", &song.number, &song.title);
    let label = Label::new(Some(&text));
    button.set_child(Some(&label));

    let song_full_path = format!("{}/{}", config.song_directory, song.file);

    button.connect_clicked(move |_| {
        let song_status = player::play_song(&song_full_path.clone());

        match song_status {
            Some(_) => (),
            None => std::process::exit(1),
        }
    });

    button
}
