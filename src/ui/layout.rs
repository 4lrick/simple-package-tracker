use adw::gtk::{Application, Box, Button, ListBox, Orientation, ScrolledWindow};
use adw::{prelude::*, ApplicationWindow, HeaderBar};

pub fn create_main_content(
    header: HeaderBar,
    scroll_window: ScrolledWindow,
    button: Button,
    package_rows: ListBox,
) -> Box {
    let content = Box::new(Orientation::Vertical, 15);
    content.append(&header);
    content.append(&scroll_window);
    content.append(&button);
    content.append(&package_rows);

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
