use adw::gtk::glib;
use adw::gtk::{Application, TextView};
use adw::{prelude::*, NavigationPage, NavigationView};
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
        let nav_view = NavigationView::new();
        let scroll_window = create_input_area(text_field.clone());
        let (button, package_rows) = create_tracking_area(text_field, nav_view.clone());
        let content = create_main_content(header, scroll_window, button, package_rows);
        let root_page = NavigationPage::builder().child(&content).build();
        let window = create_main_window(app.clone(), nav_view.clone());

        nav_view.push(&root_page);
        window.present();
    });

    app.run()
}
