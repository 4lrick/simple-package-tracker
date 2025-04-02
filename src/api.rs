use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(default)]
    pub shipment: Option<Shipment>,
}

#[derive(Debug, Deserialize)]
pub struct Shipment {
    #[serde(default)]
    pub event: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub date: String,
}

fn parse_tracking_info(json: &str) -> Option<String> {
    let api_response: ApiResponse = serde_json::from_str(json).ok()?;
    let shipment = api_response.shipment?;
    let latest = shipment.event.first()?;

    Some(format!("{}\n{}", latest.label, latest.date))
}

pub fn process_tracking_numbers(input: &str) -> Vec<String> {
    let mut results = Vec::new();

    for number in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        match fetch_tracking_info(number).map(|body| parse_tracking_info(&body)) {
            Ok(Some(parsed)) => results.push(parsed),
            Ok(None) => results.push(format!("No data for {}", number)),
            Err(e) => {
                eprintln!("API Error {}: {}", number, e);
                results.push(format!("Error for {}", number));
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
