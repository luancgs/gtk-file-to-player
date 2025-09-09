use crate::{
    config::AppConfig,
    database::Database,
    ui::{NotificationSystem, SongButton},
};

use gtk::{
    Application, ApplicationWindow, Entry, ListBox, Orientation, ScrolledWindow, prelude::*,
};
use std::{cell::RefCell, rc::Rc};

pub struct MainWindow {
    window: ApplicationWindow,
    search_entry: Entry,
    list_box: Rc<RefCell<ListBox>>,
    database: Rc<RefCell<Database>>,
    config: AppConfig,
}

impl MainWindow {
    pub fn new(
        app: &Application,
        config: &AppConfig,
        database: Rc<RefCell<Database>>,
    ) -> Result<Self, String> {
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
            .title(&config.window_title)
            .default_width(960)
            .default_height(540)
            .child(&main_box)
            .build();

        let main_window = MainWindow {
            window,
            search_entry,
            list_box,
            database,
            config: config.clone(),
        };

        main_window.connect_signals(app);
        Ok(main_window)
    }

    fn connect_signals(&self, app: &Application) {
        let database = Rc::clone(&self.database);
        let list_box = Rc::clone(&self.list_box);
        let config = self.config.clone();
        let app_weak = app.downgrade();

        self.search_entry.connect_changed(move |entry| {
            let text = entry.text().to_lowercase();

            // Clear current list
            while let Some(child) = list_box.borrow().first_child() {
                list_box.borrow().remove(&child);
            }

            // Get and display results
            match database.borrow().search_songs(&text) {
                Ok(results) => {
                    for song in results.iter() {
                        let button = SongButton::new(&config, song, app_weak.clone());
                        list_box.borrow().append(button.widget());
                    }
                }
                Err(e) => {
                    eprintln!("Database error: {}", e);
                    if let Some(app) = app_weak.upgrade() {
                        NotificationSystem::send_error(&app, &format!("Database error: {}", e));
                    }
                }
            }
        });
    }

    pub fn present(&self) {
        self.window.present();
    }
}
