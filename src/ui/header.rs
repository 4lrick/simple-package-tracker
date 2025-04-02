use adw::{HeaderBar, WindowTitle};

pub fn create_header() -> HeaderBar {
    let header = HeaderBar::builder()
        .title_widget(&WindowTitle::new("Simple Package Tracker", ""))
        .show_end_title_buttons(true)
        .build();

    return header;
}
