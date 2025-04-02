use dotenv::dotenv;
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
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct Timeline {
    #[serde(rename = "shortLabel")]
    pub short_label: String,
    #[serde(rename = "longLabel")]
    pub long_label: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub status: bool,
}

#[derive(Debug)]
pub struct TrackingInfo {
    pub id_ship: String,
    pub label: String,
    pub product: String,
    pub events: Vec<Event>,
    pub timeline: Vec<Timeline>,
}

fn parse_tracking_info(json: &str) -> Option<TrackingInfo> {
    let api_response: ApiResponse = serde_json::from_str(json).ok()?;
    let shipment = api_response.shipment?;
    let latest = shipment.event.first()?;

    Some(TrackingInfo {
        id_ship: shipment.id_ship.clone(),
        label: latest.label.clone(),
        product: shipment.product.clone(),
        events: shipment.event,
        timeline: shipment.timeline,
    })
}

pub fn process_tracking_numbers(input: &str) -> Vec<TrackingInfo> {
    let mut results = Vec::new();

    for number in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        match fetch_tracking_info(number).map(|body| parse_tracking_info(&body)) {
            Ok(Some(info)) => results.push(info),
            Ok(None) => results.push(TrackingInfo {
                id_ship: number.to_string(),
                label: "No data for this package".to_string(),
                product: "Unknown".to_string(),
                events: Vec::new(),
                timeline: Vec::new(),
            }),
            Err(e) => {
                eprintln!("API Error {}: {}", number, e);
                results.push(TrackingInfo {
                    id_ship: number.to_string(),
                    label: "Error: ".to_string() + &e.to_string(),
                    product: "Unknown".to_string(),
                    events: Vec::new(),
                    timeline: Vec::new(),
                });
            }
        }
    }

    return results;
}

pub fn fetch_tracking_info(tracking_number: &str) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let base_url = std::env::var("API_URL").expect("API_URL must be set.");
    let okapi_key = std::env::var("OKAPI_KEY").expect("OKAPI_KEY must be set.");
    let tracking_url = format!("{}/idships/{}", base_url, tracking_number);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&tracking_url)
        .header("Accept", "application/json")
        .header("X-Okapi-Key", &okapi_key)
        .send()?;

    let body = response.text()?;
    println!("BODY: {}", body);
    Ok(body)
}
