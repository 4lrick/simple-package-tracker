use adw::gtk::{glib, Application, Box, HeaderBar, ScrolledWindow, PolicyType, Orientation, Align};
use adw::{gio, prelude::*, NavigationPage, NavigationView, WindowTitle, ApplicationWindow};

mod api;
mod storage;
mod home_page;
mod details_page;

use home_page::tracking_input::create_input_area;
use home_page::tracking_list::create_tracking_area;

#[tokio::main]
async fn main() -> glib::ExitCode {
    gio::resources_register_include!("simple_package_tracker.gresource")
        .expect("Failed to register embedded resources");

    let app = Application::builder()
        .application_id("io.github.alrick.simple_package_tracker")
        .build();

    app.connect_startup(|_| {
        adw::init().expect("Failed to initialize libadwaita");
    });

    app.connect_activate(|app| {
        let header = HeaderBar::builder()
            .title_widget(&WindowTitle::new("Simple Package Tracker", ""))
            .show_title_buttons(true)
            .build();

        let nav_view = NavigationView::new();
        let (tracking_input_window, text_field) = create_input_area();
        let (track_button, package_rows) = create_tracking_area(text_field, nav_view.clone());
        
        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(15)
            .build();
            
        let main_components = Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::Center)
            .spacing(50)
            .margin_start(20)
            .margin_end(20)
            .margin_top(20)
            .margin_bottom(20)
            .build();

        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vexpand(true)
            .child(&main_components)
            .build();

        main_components.append(&tracking_input_window);
        main_components.append(&track_button);
        main_components.append(&package_rows);

        content.append(&header);
        content.append(&scrolled_window);

        let root_page = NavigationPage::builder()
            .child(&content)
            .title("Simple Package Tracker")
            .build();
            
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Simple Package Tracker")
            .default_width(800)
            .default_height(1100)
            .build();

        window.set_content(Some(&nav_view));
        nav_view.push(&root_page);
        window.present();
    });

    app.run()
}
