use adw::gtk::glib;
use adw::gtk::{Application, TextView};
use adw::prelude::*;
mod api;
mod ui;
use ui::{
    header::create_header, input::create_input_area, layout::create_main_content,
    layout::create_main_window, tracker::create_tracking_area,
};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.spt.simplepackagetracker")
        .build();

    app.connect_startup(|_| {
        adw::init().expect("Failed to initialize libadwaita");
    });

    app.connect_activate(|app| {
        let header = create_header();
        let text_field = TextView::new();
        let scroll_window = create_input_area(text_field.clone());
        let (button, tracking_label) = create_tracking_area(text_field);
        let content = create_main_content(header, scroll_window, button, tracking_label);
        let window = create_main_window(app.clone(), content);

        window.present();
    });

    app.run()
}
