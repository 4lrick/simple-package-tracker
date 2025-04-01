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
        let mut results = Vec::new();

        for line in text.lines() {
            let is_valid = line.trim().chars().all(|c| c.is_ascii_digit()) && !line.is_empty();
            if is_valid {
                results.push(format!("Tracking: {}", line));
            }
        }

        tracking_label_cloned.set_text(&results.join("\n"));
        println!("{}", results.join("\n"));
    });

    return (button, tracking_label);
}
