use crate::api::process_tracking_numbers;
use adw::gtk::{Box, Button, Orientation, TextView};
use adw::prelude::*;
use adw::ActionRow;

pub fn create_package_rows(input: &str) -> Vec<ActionRow> {
    let infos = process_tracking_numbers(input);

    infos
        .into_iter()
        .map(|info| {
            let package = ActionRow::builder()
                .title(&info.id_ship)
                .subtitle(&info.label)
                .activatable(true)
                .build();

            return package;
        })
        .collect()
}

pub fn create_tracking_area(text_field: TextView) -> (Button, Box) {
    let text_field_cloned = text_field.clone();
    let package_rows = Box::new(Orientation::Vertical, 6);
    let package_rows_cloned = package_rows.clone();

    let button = Button::with_label("Track");
    button.connect_clicked(move |_| {
        let tf_buff = text_field_cloned.buffer();
        let text = tf_buff.text(&tf_buff.start_iter(), &tf_buff.end_iter(), false);

        for package in create_package_rows(&text) {
            package_rows_cloned.append(&package);
        }
    });

    return (button, package_rows);
}
