use serde::Deserialize;
use thiserror::Error;
use chrono;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: TrackingData,
}

#[derive(Debug, Deserialize)]
pub struct TrackingData {
    pub trackings: Vec<Tracking>,
}

#[derive(Debug, Deserialize)]
pub struct Tracking {
    pub tracker: Tracker,
    pub shipment: Shipment,
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct Tracker {
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
}

#[derive(Debug, Deserialize)]
pub struct Shipment {
    #[serde(rename = "statusMilestone")]
    pub status_milestone: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Event {
    pub status: Option<String>,
    #[serde(rename = "occurrenceDatetime", deserialize_with = "deserialize_datetime")]
    pub occurrence_datetime: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "statusMilestone")]
    pub status_milestone: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusMilestone {
    Pending,
    InfoReceived,
    InTransit,
    OutForDelivery,
    FailedAttempt,
    AvailableForPickup,
    Delivered,
    Exception,
}

impl StatusMilestone {
    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => Self::Pending,
            "info_received" => Self::InfoReceived,
            "in_transit" => Self::InTransit,
            "out_for_delivery" => Self::OutForDelivery,
            "failed_attempt" => Self::FailedAttempt,
            "available_for_pickup" => Self::AvailableForPickup,
            "delivered" => Self::Delivered,
            "exception" => Self::Exception,
            _ => Self::Pending,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::InfoReceived => "Information Received",
            Self::InTransit => "In Transit",
            Self::OutForDelivery => "Out for Delivery",
            Self::FailedAttempt => "Failed Attempt",
            Self::AvailableForPickup => "Available for Pickup",
            Self::Delivered => "Delivered",
            Self::Exception => "Exception",
        }
    }

    pub fn order(&self) -> u8 {
        match self {
            Self::Pending => 0,
            Self::InfoReceived => 1,
            Self::InTransit => 2,
            Self::OutForDelivery => 3,
            Self::FailedAttempt => 4,
            Self::AvailableForPickup => 5,
            Self::Delivered => 6,
            Self::Exception => 7,
        }
    }

    pub fn is_completed_at(&self, current: Self) -> bool {
        if current == Self::Delivered {
            true
        } else {
            self.order() < current.order()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackingInfo {
    pub id_ship: String,
    pub label: String,
    pub status: String,
    pub events: Vec<Event>,
    pub timeline: Vec<Timeline>,
    pub url: Option<String>,
    pub has_error: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Timeline {
    pub short_label: String,
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub errors: Vec<ApiError>,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Error)]
pub enum TrackingError {
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Invalid tracking number: {0}")]
    InvalidTrackingNumber(String),
    #[error("No tracking data available")]
    NoTrackingData,
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Server error: {0}")]
    ServerError(String),
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
        return Ok(dt.with_timezone(&chrono::Utc));
    }
    
    chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
        .map(|naive_dt| chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc))
        .map_err(|e| serde::de::Error::custom(format!("Failed to parse datetime: {}", e)))
} 