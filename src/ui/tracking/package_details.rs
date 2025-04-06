use crate::api::{process_tracking_numbers, TrackingInfo};
use adw::{
    glib,
    gtk::{
        Align, Box, Button, Image, Label, ListBox, Orientation, PolicyType, ProgressBar,
        ScrolledWindow, Separator,
    },
    prelude::*,
    ActionRow, HeaderBar, NavigationPage, Spinner, ToolbarView,
};
use chrono::DateTime;

pub fn create_events_history(info: &TrackingInfo, events_box: Box) -> Box {
    let events_label = Label::builder()
        .label("History")
        .css_classes(vec!["title-3"])
        .halign(Align::Start)
        .margin_bottom(20)
        .build();

    let events_list = ListBox::builder()
        .css_classes(vec!["boxed-list"])
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

fn show_loading_state(info: &TrackingInfo) -> ScrolledWindow {
    let details = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .valign(Align::Center)
        .margin_bottom(20)
        .margin_top(20)
        .margin_start(20)
        .margin_end(20)
        .spacing(20)
        .build();

    let title = Label::builder()
        .label(&info.id_ship)
        .css_classes(vec!["title-1"])
        .margin_bottom(50)
        .build();

    let spinner = Spinner::builder()
        .width_request(64)
        .height_request(64)
        .build();

    details.append(&title);
    details.append(&spinner);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&details)
        .build();

    return scrolled_window;
}

fn create_details_content(info: &TrackingInfo) -> ScrolledWindow {
    let details = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .valign(Align::Center)
        .margin_bottom(20)
        .margin_top(20)
        .margin_start(20)
        .margin_end(20)
        .spacing(20)
        .build();

    let title = Label::builder()
        .label(&info.id_ship)
        .css_classes(vec!["title-1"])
        .margin_bottom(50)
        .build();

    details.append(&title);

    if info.label == "No data for this package" {
        let no_data_label = Label::builder()
            .label("No tracking information available for this package")
            .css_classes(vec!["dim-label"])
            .margin_top(20)
            .build();
        details.append(&no_data_label);
    } else {
        let events_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_top(20)
            .build();

        let events_box = create_events_history(info, events_box);
        let completed_steps = info.timeline.iter().filter(|t| t.status).count();
        let total_steps = info.timeline.len().max(1);
        let progress = completed_steps as f64 / total_steps as f64;

        let progress_bar = ProgressBar::builder()
            .fraction(progress)
            .tooltip_markup("Timeline progress")
            .height_request(6)
            .margin_top(10)
            .margin_bottom(10)
            .build();

        let status_product_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .hexpand(true)
            .build();

        if let Some(latest) = info
            .timeline
            .iter()
            .filter(|t| !t.short_label.is_empty())
            .last()
        {
            let status_label = Label::builder()
                .label(&latest.short_label)
                .halign(Align::Start)
                .hexpand(true)
                .wrap(true)
                .build();

            let product_label = Label::builder()
                .label(&format!("Product: {}", info.product))
                .halign(Align::End)
                .hexpand(true)
                .build();

            status_product_box.append(&status_label);
            status_product_box.append(&product_label);
            details.append(&status_product_box);
        }

        details.append(&Separator::new(Orientation::Horizontal));
        details.append(&progress_bar);
        details.append(&events_box);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&details)
        .build();

    return scrolled_window;
}

fn create_header(info: TrackingInfo, nav_page: &NavigationPage) -> HeaderBar {
    let header = HeaderBar::new();
    let info_id = info.id_ship.clone();
    let nav_page_clone = nav_page.clone();
    let infos = info.clone();
    let refresh_button = Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_markup("Refresh tracking information")
        .build();

    refresh_button.connect_clicked(move |button| {
        let nav_page_clone = nav_page_clone.clone();
        let info_id_clone = info_id.clone();
        let infos_clone = infos.clone();

        if let Some(toolbar) = nav_page_clone.child().and_downcast::<ToolbarView>() {
            let loading_view = show_loading_state(&infos_clone);
            toolbar.set_content(Some(&loading_view));
            button.set_sensitive(false);
        }

        glib::spawn_future_local(async move {
            let tracking_info = process_tracking_numbers(&info_id_clone).await;
            if let Some(new_info) = tracking_info.into_iter().next() {
                let toolbar = ToolbarView::new();
                let new_header = create_header(new_info.clone(), &nav_page_clone);
                let details = create_details_content(&new_info);

                toolbar.set_content(Some(&details));
                toolbar.add_top_bar(&new_header);
                nav_page_clone.set_child(Some(&toolbar));
            }
        });
    });

    if let Some(url) = info.url {
        let image = Image::builder()
            .resource("/io/github/alrick/simple_package_tracker/icons/external-link-symbolic.svg")
            .build();

        let url_button = Button::builder()
            .child(&image)
            .css_classes(vec!["flat"])
            .tooltip_markup("Open tracking page in browser")
            .build();

        let url_clone = url.clone();
        url_button.connect_clicked(move |_| {
            if let Err(e) = open::that(&url_clone) {
                eprintln!("Failed to open URL: {}", e);
            }
        });

        header.pack_end(&url_button);
    }

    header.pack_start(&refresh_button);
    return header;
}

pub fn create_details_page(info: &TrackingInfo) -> NavigationPage {
    let nav_page = NavigationPage::builder()
        .title("Package Details")
        .tag(&info.id_ship)
        .build();

    let toolbar = ToolbarView::new();
    let header = create_header(info.clone(), &nav_page);
    let details = create_details_content(info);

    toolbar.set_content(Some(&details));
    toolbar.add_top_bar(&header);
    nav_page.set_child(Some(&toolbar));

    nav_page
}
