use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box, Button, HeaderBar, Label, Orientation,
    ScrolledWindow, TextView,
};
use gtk4::{self as gtk};

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

        let header_bar = HeaderBar::builder().name("Simple Package Tracker").build();

        let vbox = Box::new(Orientation::Vertical, 15);
        let text_field = TextView::new();
        let scroll_window = ScrolledWindow::builder().min_content_height(300).build();

        let text_field_cloned = text_field.clone();
        let tracking_label = Label::builder().build();
        let tracking_label_cloned = tracking_label.clone();
        let button = Button::with_label("Track");

        button.connect_clicked(move |_| {
            let tf_buff = text_field_cloned.buffer();
            let text_field_output = tf_buff.text(&tf_buff.start_iter(), &tf_buff.end_iter(), false);
            let mut tracking_number = Vec::new();
            for line in text_field_output.lines() {
                let is_clean: bool =
                    line.trim().chars().all(|c| c.is_ascii_digit()) && !line.is_empty();
                if !is_clean {
                    continue;
                }

                let message = format!("Tracking: {}", line);
                tracking_number.push(message);
            }
            tracking_label_cloned.set_text(&tracking_number.join("\n"));
            println!("{}", tracking_number.join("\n"));
        });

        scroll_window.set_child(Some(&text_field));
        vbox.append(&scroll_window);
        vbox.append(&button);
        vbox.append(&tracking_label);
        window.set_titlebar(Some(&header_bar));
        window.set_child(Some(&vbox));

        window.present();
    });

    app.run()
}
