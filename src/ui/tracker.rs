use crate::api::{process_tracking_numbers, TrackingInfo};
use adw::gtk::{Align, Box, Button, Label, ListBox, Orientation, TextView};
use adw::{prelude::*, HeaderBar, NavigationView, ToolbarView};
use adw::{ActionRow, NavigationPage};

pub fn create_details_page(info: &TrackingInfo) -> NavigationPage {
    let nav_page = NavigationPage::builder()
        .title("Package Details")
        .tag(&info.id_ship)
        .build();

    let toolbar = ToolbarView::new();
    let header = HeaderBar::new();
    let button = Button::builder().label("Testing Button").build();
    button.connect_clicked(move |_| {
        println!("Clicked on button");
    });

    let details = Box::new(Orientation::Vertical, 6);

    details.append(&Label::new(Some(&format!("ID: {}", &info.id_ship))));
    details.append(&Label::new(Some(&format!(
        "Type of tracking: {}",
        &info.product
    ))));

    for event in &info.events {
        details.append(&Label::new(Some(&format!(
            "Event on {}: {}",
            event.date, event.label
        ))));
    }

    if !info.timeline.is_empty() {
        details.append(&Label::new(Some("Timeline steps:")));
        for step in &info.timeline {
            let date_str = if step.date.is_empty() {
                "".to_string()
            } else {
                format!(" on {}", step.date)
            };

            let label = format!(
                "- {} - {} - {} (Status: {})",
                step.short_label,
                step.long_label,
                date_str,
                if step.status { "✔" } else { "✘" }
            );
            details.append(&Label::new(Some(&label)));
        }
    }

    toolbar.set_content(Some(&details));
    toolbar.add_top_bar(&header);
    nav_page.set_child(Some(&toolbar));

    return nav_page;
}

pub fn create_package_rows(input: &str, nav_view: &NavigationView) -> Vec<ActionRow> {
    let infos = process_tracking_numbers(input);

    infos
        .into_iter()
        .map(|info| {
            let package = ActionRow::builder()
                .title(&info.id_ship)
                .subtitle(&info.label)
                .activatable(true)
                .build();

            let delete_btn = Button::builder().icon_name("user-trash-symbolic").build();
            let package_clone = package.clone();
            delete_btn.connect_clicked(move |_| {
                if let Some(parent) = package_clone.parent() {
                    if let Some(box_container) = parent.downcast_ref::<ListBox>() {
                        box_container.remove(&package_clone);
                    }
                }
            });

            let nav_view_clone = nav_view.clone();
            let nav_page_clone = create_details_page(&info);
            package.connect_activated(move |_| {
                println!("Clicked on package: {}", info.id_ship);
                nav_view_clone.push(&nav_page_clone);
            });

            package.add_suffix(&delete_btn);

            return package;
        })
        .collect()
}

pub fn create_tracking_area(text_field: TextView, nav_view: NavigationView) -> (Button, ListBox) {
    let text_field_cloned = text_field.clone();
    let package_rows = ListBox::new();
    let package_rows_cloned = package_rows.clone();
    let track_button = Button::builder()
        .label("Track")
        .width_request(200)
        .height_request(25)
        .halign(Align::Center)
        .css_classes(vec!["suggested-action", "pill"])
        .build();

    track_button.connect_clicked(move |_| {
        let tf_buff = text_field_cloned.buffer();
        let text = tf_buff.text(&tf_buff.start_iter(), &tf_buff.end_iter(), false);

        for package in create_package_rows(&text, &nav_view) {
            package_rows_cloned.append(&package);
        }
    });

    return (track_button, package_rows);
}
