use adw::gtk::{ScrolledWindow, TextView};

pub fn create_input_area(text_field: TextView) -> ScrolledWindow {
    let scroll_window = ScrolledWindow::builder()
        .min_content_height(300)
        .child(&text_field)
        .build();

    return scroll_window;
}
