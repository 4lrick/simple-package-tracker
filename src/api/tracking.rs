use super::models::*;
use dotenvy_macro::dotenv;
use std::time::Duration;
use std::sync::Arc;
use reqwest::Client;

const SHIP24_TRACKING_URL: &str = "https://www.ship24.com/tracking?p=";
const BASE_URL: &str = "https://api.ship24.com/public/v1";

pub struct TrackingClient {
    client: Arc<Client>,
    api_key: String,
}

impl TrackingClient {
    pub fn new() -> Self {
        let client = Client::new();
        let api_key = dotenv!("API_KEY").to_string();
        
        Self {
            client: Arc::new(client),
            api_key,
        }
    }

    pub async fn process_tracking_numbers(&self, input: &str) -> Vec<TrackingInfo> {
        let numbers: Vec<_> = input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect();

        let mut results = Vec::with_capacity(numbers.len());
        let mut tasks = Vec::new();

        for chunk in numbers.chunks(10) {
            let mut chunk_tasks = Vec::new();
            for &number in chunk {
                let number = number.to_string();
                let client = self.client.clone();
                let api_key = self.api_key.clone();
                chunk_tasks.push(tokio::spawn(async move {
                    let tracking_url = format!("{}/trackers/track", BASE_URL);
                    let response = client
                        .post(&tracking_url)
                        .header("Authorization", format!("Bearer {}", api_key))
                        .header("Content-Type", "application/json")
                        .json(&serde_json::json!({
                            "trackingNumber": &number,
                            "settings": {
                                "restrictTrackingToCourierCode": false
                            }
                        }))
                        .send()
                        .await;

                    match response {
                        Ok(resp) => {
                            let status = resp.status();
                            match resp.text().await {
                                Ok(body) => {
                                    match status {
                                        reqwest::StatusCode::OK | reqwest::StatusCode::CREATED => {
                                            match parse_tracking_info(&body) {
                                                Some(info) => info,
                                                None => TrackingInfo {
                                                    id_ship: number.clone(),
                                                    label: "Status unknown".to_string(),
                                                    status: "Unknown".to_string(),
                                                    events: Vec::new(),
                                                    timeline: Vec::new(),
                                                    url: None,
                                                    has_error: true,
                                                    error_message: Some("Failed to parse tracking data".to_string()),
                                                },
                                            }
                                        },
                                        _ => {
                                            let error_message = if let Ok(error_response) = serde_json::from_str::<ApiErrorResponse>(&body) {
                                                if let Some(error) = error_response.errors.first() {
                                                    error.message.clone().unwrap_or_else(|| error.code.clone())
                                                } else {
                                                    format!("API error: {}", status)
                                                }
                                            } else {
                                                format!("API error: {}", status)
                                            };
                                            TrackingInfo {
                                                id_ship: number,
                                                label: "Status unknown".to_string(),
                                                status: "Unknown".to_string(),
                                                events: Vec::new(),
                                                timeline: Vec::new(),
                                                url: None,
                                                has_error: true,
                                                error_message: Some(error_message),
                                            }
                                        }
                                    }
                                },
                                Err(e) => TrackingInfo {
                                    id_ship: number,
                                    label: "Status unknown".to_string(),
                                    status: "Unknown".to_string(),
                                    events: Vec::new(),
                                    timeline: Vec::new(),
                                    url: None,
                                    has_error: true,
                                    error_message: Some(format!("Network error: {}", e)),
                                },
                            }
                        },
                        Err(e) => TrackingInfo {
                            id_ship: number,
                            label: "Status unknown".to_string(),
                            status: "Unknown".to_string(),
                            events: Vec::new(),
                            timeline: Vec::new(),
                            url: None,
                            has_error: true,
                            error_message: Some(format!("Network error: {}", e)),
                        },
                    }
                }));
            }

            tasks.extend(chunk_tasks);
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        for task in tasks {
            if let Ok(info) = task.await {
                results.push(info);
            }
        }

        results
    }
}

pub fn parse_tracking_info(json: &str) -> Option<TrackingInfo> {
    let api_response: ApiResponse = match serde_json::from_str(json) {
        Ok(resp) => resp,
        Err(e) => {
            return Some(TrackingInfo {
                id_ship: "".to_string(),
                label: "".to_string(),
                status: "".to_string(),
                events: Vec::new(),
                timeline: Vec::new(),
                url: None,
                has_error: true,
                error_message: Some(e.to_string()),
            });
        },
    };

    let tracking = match api_response.data.trackings.first() {
        Some(tracking) => tracking,
        None => return None,
    };

    if tracking.events.is_empty() {
        return Some(TrackingInfo {
            id_ship: tracking.tracker.tracking_number.clone(),
            label: "No tracking data available".to_string(),
            status: "Unknown".to_string(),
            events: Vec::new(),
            timeline: Vec::new(),
            url: None,
            has_error: true,
            error_message: Some("No tracking data available".to_string()),
        });
    }
    
    let shipment = &tracking.shipment;
    let current_milestone_enum = StatusMilestone::from_str(&shipment.status_milestone);
    
    let mut timeline: Vec<Timeline> = tracking.events.iter()
        .map(|event| {
            let status_text = if let Some(status) = &event.status {
                status.clone()
            } else {
                match StatusMilestone::from_str(&event.status_milestone) {
                    StatusMilestone::Delivered => "Package has been delivered".to_string(),
                    StatusMilestone::InTransit => "Package is in transit".to_string(),
                    StatusMilestone::InfoReceived => "Package information received".to_string(),
                    StatusMilestone::Exception => "Package has an exception".to_string(),
                    StatusMilestone::AvailableForPickup => "Package is available for pickup".to_string(),
                    StatusMilestone::FailedAttempt => "Delivery attempt failed".to_string(),
                    StatusMilestone::OutForDelivery => "Package is out for delivery".to_string(),
                    StatusMilestone::Pending => "Package status is pending".to_string(),
                }
            };

            let step_status = StatusMilestone::from_str(&event.status_milestone).is_completed_at(current_milestone_enum);
            
            Timeline {
                short_label: status_text,
                status: step_status,
            }
        })
        .collect();

    timeline.sort_by(|a, b| {
        let a_milestone = tracking.events.iter()
            .find(|e| e.status.as_ref() == Some(&a.short_label))
            .map(|e| StatusMilestone::from_str(&e.status_milestone))
            .unwrap_or(StatusMilestone::Pending);
        let b_milestone = tracking.events.iter()
            .find(|e| e.status.as_ref() == Some(&b.short_label))
            .map(|e| StatusMilestone::from_str(&e.status_milestone))
            .unwrap_or(StatusMilestone::Pending);
        a_milestone.order().cmp(&b_milestone.order())
    });

    let mut seen_milestones = std::collections::HashSet::new();
    timeline.retain(|step| {
        let milestone = tracking.events.iter()
            .find(|e| e.status.as_ref() == Some(&step.short_label))
            .map(|e| StatusMilestone::from_str(&e.status_milestone))
            .unwrap_or(StatusMilestone::Pending);
        seen_milestones.insert(milestone)
    });

    for step in timeline.iter_mut() {
        let step_milestone = tracking.events.iter()
            .find(|e| e.status.as_ref() == Some(&step.short_label))
            .map(|e| StatusMilestone::from_str(&e.status_milestone))
            .unwrap_or(StatusMilestone::Pending);
        step.status = step_milestone.is_completed_at(current_milestone_enum);
    }

    let highest_milestone = tracking.events.iter()
        .map(|e| StatusMilestone::from_str(&e.status_milestone))
        .max_by_key(|m| m.order())
        .unwrap_or(current_milestone_enum);
    
    let label = {
        tracking.events.iter()
            .filter(|e| StatusMilestone::from_str(&e.status_milestone) == highest_milestone)
            .next()
            .and_then(|e| e.status.clone())
            .unwrap_or_else(|| {
                match highest_milestone {
                    StatusMilestone::Delivered => "Package has been delivered".to_string(),
                    StatusMilestone::InTransit => "Package is in transit".to_string(),
                    StatusMilestone::InfoReceived => "Package information received".to_string(),
                    StatusMilestone::Exception => "Package has an exception".to_string(),
                    StatusMilestone::AvailableForPickup => "Package is available for pickup".to_string(),
                    StatusMilestone::FailedAttempt => "Delivery attempt failed".to_string(),
                    StatusMilestone::OutForDelivery => "Package is out for delivery".to_string(),
                    StatusMilestone::Pending => "Package status is pending".to_string(),
                }
            })
    };

    let info = TrackingInfo {
        id_ship: tracking.tracker.tracking_number.clone(),
        label,
        status: highest_milestone.to_string().to_string(),
        events: tracking.events.clone(),
        timeline,
        url: Some(format!("{}{}", SHIP24_TRACKING_URL, tracking.tracker.tracking_number)),
        has_error: false,
        error_message: None,
    };

    return Some(info);
} 