use super::package_details::create_details_page;
use crate::api::process_tracking_numbers;
use crate::storage::{load_tracking_numbers, save_tracking_numbers};
use adw::glib;
use adw::{
    gtk::{
        Align, Box, Button, Frame, Label, ListBox, Orientation, ScrolledWindow, TextView,
        ToggleButton,
    },
    prelude::*,
    ActionRow, AlertDialog, NavigationView, ResponseAppearance, Spinner, StatusPage,
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

fn clean_numbers_list(input: &str) -> Vec<String> {
    let existing_numbers = load_tracking_numbers();
    let new_numbers: Vec<String> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(String::from)
        .filter(|num| !existing_numbers.contains(num))
        .collect();

    let all_numbers = if !input.is_empty() {
        let mut combined = existing_numbers;
        combined.extend(new_numbers);
        combined
    } else {
        existing_numbers
    };

    return all_numbers;
}

fn show_loading_state() -> Box {
    let loading_box = Box::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .halign(Align::Center)
        .vexpand(false)
        .spacing(10)
        .height_request(440)
        .build();

    let loading_spinner = Spinner::builder()
        .height_request(64)
        .width_request(64)
        .vexpand(true)
        .valign(Align::End)
        .build();

    let loading_label = Label::builder()
        .label("Loading packages...")
        .css_classes(vec!["dim-label"])
        .vexpand(true)
        .valign(Align::Start)
        .build();

    loading_box.append(&loading_spinner);
    loading_box.append(&loading_label);
    return loading_box;
}

async fn create_package_rows(
    input: &str,
    nav_view: &NavigationView,
    frame: &Frame,
    no_package_title: &StatusPage,
) {
    frame.set_child(Some(&show_loading_state()));

    let list = ListBox::builder().css_classes(vec!["boxed-list"]).build();
    let all_numbers = clean_numbers_list(input);

    let tracking_info = process_tracking_numbers(&all_numbers.join("\n")).await;
    let numbers: Vec<String> = tracking_info
        .iter()
        .map(|info| info.id_ship.clone())
        .collect();
    let _ = save_tracking_numbers(&numbers);
    let scrolled_window = ScrolledWindow::builder()
        .child(&list)
        .height_request(440)
        .vexpand(false)
        .build();
    frame.set_child(Some(&scrolled_window));
    for info in tracking_info {
        let package = ActionRow::builder()
            .title(&info.id_ship)
            .subtitle(&info.label)
            .activatable(true)
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

        let nav_view_clone = nav_view.clone();
        let nav_page_clone = create_details_page(&info);
        package.connect_activated(move |_| {
            nav_view_clone.push(&nav_page_clone);
        });

        package.add_suffix(&delete_btn);
        list.append(&package);
    }
    if list.first_child().is_none() {
        frame.set_child(Some(no_package_title));
    }
}

async fn refresh_tracking_info(
    _package_rows: &ListBox,
    nav_view: &NavigationView,
    frame: &Frame,
    no_package_title: &StatusPage,
    refresh_button: Button,
) {
    let numbers = load_tracking_numbers();

    if !numbers.is_empty() {
        frame.set_child(Some(&show_loading_state()));
        let input = numbers.join("\n");
        create_package_rows(&input, nav_view, frame, no_package_title).await;
    } else {
        frame.set_child(Some(no_package_title));
    }
    refresh_button.set_sensitive(true);
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

    let _scrolled_window = ScrolledWindow::builder()
        .child(&package_rows)
        .height_request(440)
        .vexpand(false)
        .build();

    let frame = Frame::builder()
        .child(&no_package_title)
        .css_classes(vec!["boxed-list"])
        .build();

    let nav_view_clone = nav_view.clone();
    let frame_clone = frame.clone();
    let no_package_title_clone = no_package_title.clone();

    glib::spawn_future_local(async move {
        let saved_numbers = load_tracking_numbers();
        if !saved_numbers.is_empty() {
            let input = saved_numbers.join("\n");
            create_package_rows(
                &input,
                &nav_view_clone,
                &frame_clone,
                &no_package_title_clone,
            )
            .await;
        }
    });

    let package_rows_for_refresh = package_rows.clone();
    let frame_for_refresh = frame.clone();
    let nav_view_for_refresh = nav_view.clone();
    let no_package_title_for_refresh = no_package_title.clone();

    refresh_button.connect_clicked(move |button| {
        button.set_sensitive(false);
        let package_rows = package_rows_for_refresh.clone();
        let nav_view = nav_view_for_refresh.clone();
        let frame = frame_for_refresh.clone();
        let no_package_title = no_package_title_for_refresh.clone();
        let button_clone = button.clone();

        glib::spawn_future_local(async move {
            refresh_tracking_info(
                &package_rows,
                &nav_view,
                &frame,
                &no_package_title,
                button_clone,
            )
            .await;
        });
    });

    let package_area = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(70)
        .build();

    let _package_rows_cloned = package_rows.clone();
    let frame_cloned = frame.clone();
    track_button.connect_clicked(move |_| {
        let tf_buff = text_field_cloned.buffer();
        let text = tf_buff.text(&tf_buff.start_iter(), &tf_buff.end_iter(), false);
        let nav_view = nav_view.clone();
        let frame_cloned = frame_cloned.clone();
        let no_package_title = no_package_title.clone();

        glib::spawn_future_local(async move {
            create_package_rows(&text, &nav_view, &frame_cloned, &no_package_title).await;
        });
    });

    title_container.append(&tracked_package_title);
    title_container.append(&refresh_button);
    package_area.append(&title_container);
    package_area.append(&frame);

    return (track_button, package_area);
}
