use adw::gtk::{Frame, ScrolledWindow, TextView, WrapMode};

pub fn create_input_area() -> (Frame, TextView) {
    let text_field = TextView::builder()
        .bottom_margin(12)
        .left_margin(12)
        .right_margin(12)
        .top_margin(12)
        .wrap_mode(WrapMode::Char)
        .build();

    let scroll_window = ScrolledWindow::builder()
        .child(&text_field)
        .height_request(180)
        .width_request(600)
        .build();

    let frame = Frame::builder().child(&scroll_window).build();

    return (frame, text_field);
}
