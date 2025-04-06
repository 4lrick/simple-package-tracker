#[macro_use]
extern crate dotenv_codegen;

use adw::gtk::{glib, Application};
use adw::{prelude::*, NavigationPage, NavigationView};
mod api;
mod storage;
mod ui;
use ui::{
    header::create_header, input::create_input_area, layout::create_main_content,
    layout::create_main_window, tracking::tracker::create_tracking_area,
};

#[tokio::main]
async fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("io.github.alrick.simple_package_tracker")
        .build();

    app.connect_startup(|_| {
        adw::init().expect("Failed to initialize libadwaita");
    });

    app.connect_activate(|app| {
        let header = create_header();
        let nav_view = NavigationView::new();
        let (tracking_input_window, text_field) = create_input_area();
        let (track_button, package_rows) = create_tracking_area(text_field, nav_view.clone());
        let content =
            create_main_content(header, tracking_input_window, track_button, package_rows);
        let root_page = NavigationPage::builder()
            .child(&content)
            .title("Simple Package Tracker")
            .build();
        let window = create_main_window(app.clone(), nav_view.clone());

        nav_view.push(&root_page);
        window.present();
    });

    app.run()
}
