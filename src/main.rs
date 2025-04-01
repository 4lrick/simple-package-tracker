use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, Entry};
use gtk4 as gtk;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.spt.simplepackagetracker")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .title("Simple Package Tracker")
            .build();

        let vbox = Box::new(gtk4::Orientation::Vertical, 15);

        let text_field = Entry::builder()
            .placeholder_text("Enter up to 50 tracking numbers, each per line")
            .build();

        let button = Button::with_label("Track");
        button.connect_clicked(|_| {
            // TODO: print the buffer of the input field
            eprintln!("Clicked!");
        });

        vbox.append(&text_field);
        vbox.append(&button);
        window.set_child(Some(&vbox));

        window.present();
    });

    app.run()
}

