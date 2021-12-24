use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SnapchatData {
    #[serde(rename = "Received Saved Chat History")]
    pub saved_recieved: Vec<RecievedSavedChat>,
    #[serde(rename = "Sent Saved Chat History")]
    pub sent_recieved: Vec<SentSavedChat>,
}

impl SnapchatData {
    pub fn from_reader<R: std::io::Read>(rdr: R) -> serde_json::Result<Self> {
        serde_json::from_reader(rdr)
    }
    pub fn from_str<'a>(s: &'a str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
}

//{"From": "Felix422", "Media Type": "TEXT", "Created": "2021-01-01 12:00:00 UTC", "Text": "Test Message"}
#[derive(Debug, Deserialize, Serialize)]
pub struct RecievedSavedChat {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "Media Type")]
    pub media_type: MediaType,
    #[serde(rename = "Created", with = "crate::timestamp")]
    pub created: DateTime<Utc>,
    #[serde(rename = "Text")]
    pub text: String,
}

//{"To": "Felix422", "Media Type": "TEXT", "Created": "2021-01-01 12:00:00 UTC", "Text": "Test Message"}
#[derive(Debug, Deserialize, Serialize)]
pub struct SentSavedChat {
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "Media Type")]
    pub media_type: MediaType,
    #[serde(rename = "Created", with = "crate::timestamp")]
    pub created: DateTime<Utc>,
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MediaType {
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "MEDIA")]
    Media,
    #[serde(rename = "STICKER")]
    Sticker,
    #[serde(rename = "SHARE")]
    Share,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "LOCATION")]
    Location,
}
