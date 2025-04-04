use adw::gtk::{Align, Application, Box, Button, Frame, Orientation, PolicyType, ScrolledWindow};
use adw::{prelude::*, ApplicationWindow, HeaderBar};

pub fn create_main_content(
    header: HeaderBar,
    tracking_input_window: Frame,
    track_button: Button,
    package_rows: Box,
) -> Box {
    let content = Box::new(Orientation::Vertical, 15);
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

    return content;
}

pub fn create_main_window(
    app: Application,
    content: impl IsA<adw::gtk::Widget>,
) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(&app)
        .default_width(800)
        .default_height(600)
        .content(&content)
        .build();

    return window;
}
