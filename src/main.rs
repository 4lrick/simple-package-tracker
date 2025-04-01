use adw::gtk::glib;
use adw::gtk::{Application, Box, Button, Label, Orientation, ScrolledWindow, TextView};
use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.spt.simplepackagetracker")
        .build();

    app.connect_startup(|_| {
        adw::init().expect("Failed to initialize libadwaita");
    });

    app.connect_activate(|app| {
        let header = HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("Simple Package Tracker", ""))
            .show_end_title_buttons(true)
            .build();

        let text_field = TextView::new();
        let scroll_window = ScrolledWindow::builder()
            .min_content_height(300)
            .child(&text_field)
            .build();

        let tracking_label = Label::new(None);
        let text_field_cloned = text_field.clone();
        let tracking_label_cloned = tracking_label.clone();

        let button = Button::with_label("Track");
        button.connect_clicked(move |_| {
            let tf_buff = text_field_cloned.buffer();
            let text = tf_buff.text(&tf_buff.start_iter(), &tf_buff.end_iter(), false);
            let mut results = Vec::new();

            for line in text.lines() {
                let is_valid = line.trim().chars().all(|c| c.is_ascii_digit()) && !line.is_empty();
                if is_valid {
                    results.push(format!("Tracking: {}", line));
                }
            }

            tracking_label_cloned.set_text(&results.join("\n"));
            println!("{}", results.join("\n"));
        });

        let content = Box::new(Orientation::Vertical, 15);
        content.append(&header);
        content.append(&scroll_window);
        content.append(&button);
        content.append(&tracking_label);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .content(&content)
            .build();

        window.present();
    });

    app.run()
}
