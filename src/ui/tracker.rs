use crate::api::{process_tracking_numbers, TrackingInfo};
use adw::{
    gtk::{
        Align, Box, Button, Frame, Label, ListBox, Orientation, ProgressBar, ScrolledWindow,
        SelectionMode, Separator, TextView, ToggleButton,
    },
    prelude::*,
    ActionRow, AlertDialog, HeaderBar, NavigationPage, NavigationView, ResponseAppearance,
    StatusPage, ToolbarView,
};
use chrono::DateTime;

pub fn create_events_detail(info: &TrackingInfo, events_box: Box) -> Box {
    let events_label = Label::builder()
        .label("History")
        .css_classes(vec!["title-3"])
        .halign(Align::Start)
        .margin_bottom(20)
        .build();

    let events_list = ListBox::builder()
        .css_classes(vec!["boxed-list"])
        .selection_mode(SelectionMode::None)
        .focusable(false)
        .can_focus(false)
        .build();

    let mut sorted_events: Vec<(String, String)> = Vec::new();
    for event in info.events.iter() {
        sorted_events.push((event.date.clone(), event.label.clone()));
        sorted_events.sort_by(|a, b| b.0.cmp(&a.0));
    }

    for (date, label) in sorted_events {
        if let Ok(parsed_date) = DateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S%z") {
            let formatted_date = parsed_date.format("%Y-%m-%d %H:%M").to_string();
            let row = ActionRow::builder()
                .title(&label)
                .subtitle(&formatted_date)
                .build();
            events_list.append(&row);
        }
    }
    events_box.append(&events_label);
    events_box.append(&events_list);
    return events_box;
}

pub fn create_details_page(info: &TrackingInfo) -> NavigationPage {
    let nav_page = NavigationPage::builder()
        .title("Package Details")
        .tag(&info.id_ship)
        .build();

    let toolbar = ToolbarView::new();
    let header = HeaderBar::new();
    let details = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(20)
        .build();

    let title = Label::builder()
        .label(&info.id_ship)
        .css_classes(vec!["title-1"])
        .margin_bottom(50)
        .build();

    let events_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(20)
        .build();

    let events_box = create_events_detail(info, events_box);
    let completed_steps = info.timeline.iter().filter(|t| t.status).count();
    let total_steps = info.timeline.len().max(1);
    let progress = completed_steps as f64 / total_steps as f64;

    let progress_bar = ProgressBar::builder()
        .fraction(progress)
        .height_request(6)
        .margin_top(10)
        .margin_bottom(10)
        .build();

    let status_product_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .hexpand(true)
        .build();

    let latest_timeline = info
        .timeline
        .iter()
        .filter(|t| !t.short_label.is_empty())
        .last();

    if let Some(latest) = latest_timeline {
        let status_label = Label::builder()
            .label(&latest.short_label)
            .halign(Align::Start)
            .hexpand(true)
            .build();

        let product_label = Label::builder()
            .label(&format!("Product: {}", info.product))
            .halign(Align::End)
            .build();

        status_product_box.append(&status_label);
        status_product_box.append(&product_label);
        details.append(&title);
        details.append(&status_product_box);
    }

    details.append(&Separator::new(Orientation::Horizontal));
    details.append(&progress_bar);
    details.append(&events_box);
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

    let tracked_package_title = Label::builder()
        .css_classes(vec!["title-1"])
        .label("Tracked Package(s):")
        .margin_bottom(50)
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
        .margin_top(70)
        .build();

    let package_rows_cloned = package_rows.clone();
    let frame_cloned = frame.clone();
    track_button.connect_clicked(move |_| {
        let tf_buff = text_field_cloned.buffer();
        let text = tf_buff.text(&tf_buff.start_iter(), &tf_buff.end_iter(), false);
        let new_rows = create_package_rows(&text, &nav_view, &frame_cloned, &no_package_title);

        if !new_rows.is_empty() {
            frame_cloned.set_child(Some(&scrolled_window));

            for new_row in new_rows {
                let mut exists = false;
                let mut row_opt = package_rows_cloned.first_child();

                while let Some(row) = row_opt {
                    if let Some(row) = row.downcast_ref::<ActionRow>() {
                        if row.title() == new_row.title() {
                            exists = true;
                            break;
                        }
                    }
                    row_opt = row.next_sibling();
                }
                if !exists {
                    package_rows_cloned.append(&new_row);
                }
            }
        }
    });

    package_area.append(&tracked_package_title);
    package_area.append(&frame);

    return (track_button, package_area);
}
