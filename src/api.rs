use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(default)]
    pub shipment: Option<Shipment>,
}

#[derive(Debug, Deserialize)]
pub struct Shipment {
    #[serde(rename = "idShip")]
    pub id_ship: String,
    #[serde(default)]
    pub event: Vec<Event>,
    #[serde(default)]
    pub product: String,
    #[serde(default)]
    pub timeline: Vec<Timeline>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Event {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub date: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Timeline {
    #[serde(rename = "shortLabel")]
    pub short_label: String,
    #[serde(default)]
    pub status: bool,
}

#[derive(Debug, Clone)]
pub struct TrackingInfo {
    pub id_ship: String,
    pub label: String,
    pub product: String,
    pub events: Vec<Event>,
    pub timeline: Vec<Timeline>,
    pub url: Option<String>,
}

fn parse_tracking_info(json: &str) -> Option<TrackingInfo> {
    let api_response: ApiResponse = serde_json::from_str(json).ok()?;
    let shipment = api_response.shipment?;
    let latest = shipment
        .timeline
        .iter()
        .filter(|t| !t.short_label.is_empty())
        .last()
        .map(|t| t.short_label.clone())
        .unwrap_or_else(|| "No data for this package".to_string());

    Some(TrackingInfo {
        id_ship: shipment.id_ship.clone(),
        label: latest,
        product: shipment.product.clone(),
        events: shipment.event,
        timeline: shipment.timeline,
        url: shipment.url,
    })
}

pub async fn process_tracking_numbers(input: &str) -> Vec<TrackingInfo> {
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
            chunk_tasks.push(tokio::spawn(async move {
                match fetch_tracking_info(&number).await {
                    Ok(body) => match parse_tracking_info(&body) {
                        Some(info) => info,
                        None => TrackingInfo {
                            id_ship: number,
                            label: "No data for this package".to_string(),
                            product: "Unknown".to_string(),
                            events: Vec::new(),
                            timeline: Vec::new(),
                            url: None,
                        },
                    },
                    Err(e) => TrackingInfo {
                        id_ship: number,
                        label: format!("Error: {}", e),
                        product: "Unknown".to_string(),
                        events: Vec::new(),
                        timeline: Vec::new(),
                        url: None,
                    },
                }
            }));
        }

        tasks.extend(chunk_tasks);
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }

    for task in tasks {
        if let Ok(info) = task.await {
            results.push(info);
        }
    }

    return results;
}

pub async fn fetch_tracking_info(
    tracking_number: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = dotenv!("API_URL");
    let okapi_key = dotenv!("OKAPI_KEY");
    let tracking_url = format!("{}/idships/{}", base_url, tracking_number);
    let client = reqwest::Client::new();
    let response = client
        .get(&tracking_url)
        .header("Accept", "application/json")
        .header("X-Okapi-Key", okapi_key)
        .send()
        .await?;

    let body = response.text().await?;
    Ok(body)
}
