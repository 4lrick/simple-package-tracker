use super::package_details::create_details_page;
use crate::api::process_tracking_numbers;
use crate::storage::{load_tracking_numbers, save_tracking_numbers};
use adw::{
    gtk::{
        Align, Box, Button, Frame, Label, ListBox, Orientation, ScrolledWindow, TextView,
        ToggleButton,
    },
    prelude::*,
    ActionRow, AlertDialog, NavigationView, ResponseAppearance, StatusPage,
};

fn handle_delete_numbers(
    package: &ActionRow,
    frame: &Frame,
    no_package_title: &StatusPage,
) -> AlertDialog {
    let delete_dialog = AlertDialog::builder()
        .heading("Delete?")
        .body("Are you sure you want to remove this number from the list?")
        .close_response("cancel")
        .build();

    delete_dialog.add_response("cancel", "Cancel");
    delete_dialog.add_response("remove", "Remove");
    delete_dialog.set_response_appearance("remove", ResponseAppearance::Destructive);

    if let Some(parent) = package.parent() {
        if let Some(box_container) = parent.downcast_ref::<ListBox>() {
            let box_container_clone = box_container.clone();
            let package_clone = package.clone();
            let frame_clone = frame.clone();
            let no_title_clone = no_package_title.clone();

            delete_dialog.connect_response(None, move |dialog, response| {
                if response == "remove" {
                    box_container_clone.remove(&package_clone);
                    let mut remaining_numbers = Vec::new();
                    let mut row_opt = box_container_clone.first_child();

                    while let Some(row) = row_opt {
                        if let Some(row) = row.downcast_ref::<ActionRow>() {
                            remaining_numbers.push(row.title().to_string());
                        }
                        row_opt = row.next_sibling();
                    }
                    let _ = save_tracking_numbers(&remaining_numbers);

                    if box_container_clone.first_child().is_none() {
                        frame_clone.set_child(Some(&no_title_clone));
                    }
                }
                dialog.close();
            });
        }
    }

    return delete_dialog;
}

fn create_package_rows(
    input: &str,
    nav_view: &NavigationView,
    frame: &Frame,
    no_package_title: &StatusPage,
) -> Vec<ActionRow> {
    let numbers = if !input.is_empty() {
        input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect::<Vec<_>>()
    } else {
        load_tracking_numbers()
    };

    let infos = process_tracking_numbers(&numbers.join("\n"));

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
                .tooltip_markup("Delete this package")
                .valign(Align::Center)
                .build();

            let package_clone = package.clone();
            let frame_clone = frame.clone();
            let no_title_clone = no_package_title.clone();

            delete_btn.connect_clicked(move |_| {
                let delete_dialog =
                    handle_delete_numbers(&package_clone, &frame_clone, &no_title_clone);
                delete_dialog.present(Some(&package_clone));
            });

            if info.label != "No data for this package" {
                let nav_view_clone = nav_view.clone();
                let nav_page_clone = create_details_page(&info);
                package.connect_activated(move |_| {
                    nav_view_clone.push(&nav_page_clone);
                });
            }

            package.add_suffix(&delete_btn);
            return package;
        })
        .collect()
}

fn refresh_tracking_info(
    package_rows: &ListBox,
    nav_view: &NavigationView,
    frame: &Frame,
    no_package_title: &StatusPage,
) {
    let mut numbers = Vec::new();
    let mut row_opt = package_rows.first_child();

    while let Some(row) = row_opt {
        if let Some(row) = row.downcast_ref::<ActionRow>() {
            numbers.push(row.title().to_string());
        }
        row_opt = row.next_sibling();
    }

    while let Some(child) = package_rows.first_child() {
        package_rows.remove(&child);
    }

    if !numbers.is_empty() {
        let input = numbers.join("\n");
        let new_rows = create_package_rows(&input, nav_view, frame, no_package_title);

        for row in new_rows {
            package_rows.append(&row);
        }
    }
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

    let title_container = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .hexpand(true)
        .margin_bottom(50)
        .build();

    let tracked_package_title = Label::builder()
        .css_classes(vec!["title-1"])
        .label("Tracked Package(s):")
        .hexpand(true)
        .halign(Align::Start)
        .build();

    let refresh_button = Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_markup("Refresh tracking information")
        .width_request(40)
        .height_request(25)
        .valign(Align::End)
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

    let saved_numbers = load_tracking_numbers();
    if !saved_numbers.is_empty() {
        let new_rows = create_package_rows("", &nav_view, &frame, &no_package_title);
        if !new_rows.is_empty() {
            frame.set_child(Some(&scrolled_window));
            for row in new_rows {
                package_rows.append(&row);
            }
        }
    }

    let package_rows_for_refresh = package_rows.clone();
    let frame_for_refresh = frame.clone();
    let nav_view_for_refresh = nav_view.clone();
    let no_package_title_for_refresh = no_package_title.clone();

    refresh_button.connect_clicked(move |_| {
        refresh_tracking_info(
            &package_rows_for_refresh,
            &nav_view_for_refresh,
            &frame_for_refresh,
            &no_package_title_for_refresh,
        );
    });

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
            let mut current_numbers = Vec::new();
            let mut row_opt = package_rows_cloned.first_child();

            while let Some(row) = row_opt {
                if let Some(row) = row.downcast_ref::<ActionRow>() {
                    current_numbers.push(row.title().to_string());
                }
                row_opt = row.next_sibling();
            }

            for new_row in new_rows {
                let title = new_row.title().to_string();
                if !current_numbers.contains(&title) {
                    current_numbers.push(title);
                    package_rows_cloned.append(&new_row);
                }
            }

            let _ = save_tracking_numbers(&current_numbers);
        }
    });

    title_container.append(&tracked_package_title);
    title_container.append(&refresh_button);
    package_area.append(&title_container);
    package_area.append(&frame);

    return (track_button, package_area);
}
