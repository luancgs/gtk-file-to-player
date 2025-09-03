mod config;
mod database;

use config::AppConfig;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Entry, Label, ListBox, Orientation, ScrolledWindow, glib,
};
use sqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.luancgs.file-to-player";

fn main() -> glib::ExitCode {
    // Load configuration
    let config = AppConfig::load();

    // Initialize database connection
    let connection = Rc::new(database::connect_to_database(&config.database_path).unwrap());

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(move |app| {
        build_ui(app, config.clone(), connection.clone());
    });

    // Run the application
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

    search_entry.connect_changed(move |entry| {
        let text = entry.text().to_lowercase();

        // Clear the current list
        while let Some(child) = list_box_clone.borrow().first_child() {
            list_box_clone.borrow().remove(&child);
        }

        let results = database::get_songs(&connection_clone, &text).unwrap();

        for song in results.iter() {
            let label = Label::new(Some(&song.title));
            list_box_clone.borrow().append(&label);
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
