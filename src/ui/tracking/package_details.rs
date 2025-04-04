use crate::api::{process_tracking_numbers, TrackingInfo};
use adw::{
    gtk::{
        Align, Box, Button, Label, ListBox, Orientation, PolicyType, ProgressBar, ScrolledWindow,
        Separator,
    },
    prelude::*,
    ActionRow, HeaderBar, NavigationPage, ToolbarView,
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
        details.append(&title);
        details.append(&status_product_box);
    }

    details.append(&Separator::new(Orientation::Horizontal));
    details.append(&progress_bar);
    details.append(&events_box);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&details)
        .build();

    return scrolled_window;
}

fn create_header(info: &TrackingInfo, nav_page: &NavigationPage) -> HeaderBar {
    let header = HeaderBar::new();
    let refresh_button = Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_markup("Refresh tracking information")
        .build();
    let info_clone = info.id_ship.clone();
    let nav_page_clone = nav_page.clone();

    refresh_button.connect_clicked(move |_| {
        if let Some(new_info) = process_tracking_numbers(&info_clone).into_iter().next() {
            if new_info.label != "No data for this package" {
                let toolbar = ToolbarView::new();
                let new_header = create_header(&new_info, &nav_page_clone);
                let details = create_details_content(&new_info);

                toolbar.set_content(Some(&details));
                toolbar.add_top_bar(&new_header);
                nav_page_clone.set_child(Some(&toolbar));
            }
        }
    });

    header.pack_start(&refresh_button);
    return header;
}

pub fn create_details_page(info: &TrackingInfo) -> NavigationPage {
    let nav_page = NavigationPage::builder()
        .title("Package Details")
        .tag(&info.id_ship)
        .build();

    let toolbar = ToolbarView::new();
    let header = create_header(info, &nav_page);
    let details = create_details_content(info);

    toolbar.set_content(Some(&details));
    toolbar.add_top_bar(&header);
    nav_page.set_child(Some(&toolbar));

    nav_page
}
