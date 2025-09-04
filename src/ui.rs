use gtk::{Application, ApplicationWindow, Button, Label, prelude::*};

pub fn show_error_and_exit(app: &Application, message: &str) {
    let label = Label::builder()
        .label(message)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button = Button::builder().label("OK").build();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    vbox.append(&label);
    vbox.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Error")
        .child(&vbox)
        .build();

    button.connect_clicked(|_| {
        std::process::exit(1);
    });

    window.connect_close_request(|_| {
        std::process::exit(1);
    });

    window.present();
}
