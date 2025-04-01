use adw::gtk::{Application, Box, Button, Label, Orientation, ScrolledWindow};
use adw::{prelude::*, ApplicationWindow, HeaderBar};

pub fn create_main_content(
    header: HeaderBar,
    scroll_window: ScrolledWindow,
    button: Button,
    tracking_label: Label,
) -> Box {
    let content = Box::new(Orientation::Vertical, 15);
    content.append(&header);
    content.append(&scroll_window);
    content.append(&button);
    content.append(&tracking_label);

    return content;
}

pub fn create_main_window(app: Application, content: Box) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(&app)
        .default_width(800)
        .default_height(600)
        .content(&content)
        .build();

    return window;
}
