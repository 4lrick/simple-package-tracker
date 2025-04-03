use crate::api::{process_tracking_numbers, TrackingInfo};
use adw::{
    gtk::{
        Align, Box, Button, Frame, Label, ListBox, Orientation, ScrolledWindow, TextView,
        ToggleButton,
    },
    prelude::*,
    ActionRow, AlertDialog, HeaderBar, NavigationPage, NavigationView, ResponseAppearance,
    StatusPage, ToolbarView,
};

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

pub fn create_package_rows(
    input: &str,
    nav_view: &NavigationView,
    frame: &Frame,
    no_package_title: &StatusPage,
) -> Vec<ActionRow> {
    let infos = process_tracking_numbers(input);

    infos
        .into_iter()
        .map(|info| {
            let package = ActionRow::builder()
                .title(&info.id_ship)
                .subtitle(&info.label)
                .activatable(true)
                .css_classes(vec!["property"])
                .build();

            let delete_btn = ToggleButton::builder()
                .icon_name("user-trash-symbolic")
                .valign(Align::Center)
                .build();

            let delete_dialog = AlertDialog::builder()
                .heading("Delete?")
                .body("Are you sure you want to remove this number from the list?")
                .close_response("cancel")
                .build();

            delete_dialog.add_response("cancel", "Cancel");
            delete_dialog.add_response("remove", "Remove");
            delete_dialog.set_response_appearance("remove", ResponseAppearance::Destructive);

            let package_clone = package.clone();
            let frame_clone = frame.clone();
            let no_title_clone = no_package_title.clone();

            delete_btn.connect_clicked(move |_| {
                if let Some(parent) = package_clone.parent() {
                    if let Some(box_container) = parent.downcast_ref::<ListBox>() {
                        let box_container_clone = box_container.clone();
                        let package_clone = package_clone.clone();
                        let package_clone2 = package_clone.clone();
                        let frame_clone = frame_clone.clone();
                        let no_title_clone = no_title_clone.clone();

                        delete_dialog.connect_response(None, move |dialog, response| {
                            if response == "remove" {
                                box_container_clone.remove(&package_clone);
                                if box_container_clone.first_child().is_none() {
                                    frame_clone.set_child(Some(&no_title_clone));
                                }
                            }
                            dialog.close();
                        });

                        delete_dialog.present(Some(&package_clone2));
                    }
                }
            });

            let nav_view_clone = nav_view.clone();
            let nav_page_clone = create_details_page(&info);
            package.connect_activated(move |_| {
                nav_view_clone.push(&nav_page_clone);
            });

            package.add_suffix(&delete_btn);
            return package;
        })
        .collect()
}

pub fn create_tracking_area(text_field: TextView, nav_view: NavigationView) -> (Button, Box) {
    let text_field_cloned = text_field.clone();
    let package_rows = ListBox::builder().css_classes(vec!["boxed-list"]).build();

    let track_button = Button::builder()
        .label("Track")
        .width_request(200)
        .height_request(25)
        .halign(Align::Center)
        .css_classes(vec!["suggested-action", "pill"])
        .build();

    let tracked_package_title = StatusPage::builder()
        .title("Tracked Package(s):")
        .vexpand(false)
        .build();

    let no_package_title = StatusPage::builder()
        .title("No tracked packages")
        .description("Enter one or more tracking numbers.")
        .icon_name("system-search-symbolic")
        .height_request(440)
        .build();

    let scrolled_window = ScrolledWindow::builder()
        .child(&package_rows)
        .height_request(440)
        .vexpand(false)
        .build();

    let frame = Frame::builder()
        .child(&no_package_title)
        .css_classes(vec!["boxed-list"])
        .build();

    let package_area = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();

    let package_rows_cloned = package_rows.clone();
    let frame_cloned = frame.clone();
    track_button.connect_clicked(move |_| {
        let tf_buff = text_field_cloned.buffer();
        let text = tf_buff.text(&tf_buff.start_iter(), &tf_buff.end_iter(), false);
        let packages = create_package_rows(&text, &nav_view, &frame_cloned, &no_package_title);

        if !packages.is_empty() {
            frame_cloned.set_child(Some(&scrolled_window));

            for package in packages {
                package_rows_cloned.append(&package);
            }
        }
    });

    package_area.append(&tracked_package_title);
    package_area.append(&frame);

    return (track_button, package_area);
}
