use crate::api::process_tracking_numbers;
use adw::gtk::{Button, Label, TextView};
use adw::prelude::*;

pub fn create_tracking_area(text_field: TextView) -> (Button, Label) {
    let tracking_label = Label::new(None);
    let text_field_cloned = text_field.clone();
    let tracking_label_cloned = tracking_label.clone();

    let button = Button::with_label("Track");
    button.connect_clicked(move |_| {
        let tf_buff = text_field_cloned.buffer();
        let text = tf_buff.text(&tf_buff.start_iter(), &tf_buff.end_iter(), false);
        let tracking_infos = process_tracking_numbers(&text);

        tracking_label_cloned.set_text(&tracking_infos.join("\n"));
        println!("{}", tracking_infos.join("\n"));
    });

    return (button, tracking_label);
}
