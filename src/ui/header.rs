use adw::HeaderBar;

pub fn create_header() -> HeaderBar {
    let header = HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new("Simple Package Tracker", ""))
        .show_end_title_buttons(true)
        .build();

    return header;
}

