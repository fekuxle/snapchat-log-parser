use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
//#[serde_with]
#[serde(untagged)]
pub enum Message {
    Sent {     
        #[serde(rename = "To")]
        to: String,
        #[serde(rename = "Media Type")]
        media_type: MediaType,
        #[serde(rename = "Created", with = "crate::timestamp")]
        created_at: DateTime<Utc>,
        #[serde(rename = "Text", with = "string_empty_as_none")]
        text: Option<String>,
    },
    Recieved {
        #[serde(rename = "From")]
        from: String,
        #[serde(rename = "Media Type")]
        media_type: MediaType,
        #[serde(rename = "Created", with = "crate::timestamp")]
        created_at: DateTime<Utc>,
        #[serde(rename = "Text", with = "string_empty_as_none")]
        text: Option<String>,
    }
}

impl Message {
    pub fn recipient(&self) -> String {
        match self {
            Message::Recieved { from, ..} => from.to_owned(),
            Message::Sent { to, .. } => to.to_owned(),
        }
    }
    pub fn media_type(&self) -> MediaType {
        match self {
            Self::Recieved { media_type, .. } => media_type.clone(),
            Self::Sent { media_type, .. } => media_type.clone(),
        }
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        match self {
            Self::Recieved { created_at, .. } => created_at.clone(),
            Self::Sent { created_at, .. } => created_at.clone(),
        }
    }
    pub fn text(&self) -> Option<String> {
        match self {
            Self::Recieved { text, .. } => text.clone(),
            Self::Sent { text, ..} => text.clone()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SnapchatData {
    #[serde(rename = "Received Saved Chat History")]
    pub saved_recieved: Vec<Message>,
    #[serde(rename = "Sent Saved Chat History")]
    pub saved_sent: Vec<Message>,
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
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
