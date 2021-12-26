//! All the types defined by this crate

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;
use std::collections::HashSet;

/// Enum for message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Message {
    /// Sent message
    Sent {
        /// Username of the person who sent the message
        #[serde(rename = "To")]
        to: String,
        /// Type of the message
        #[serde(rename = "Media Type")]
        media_type: MediaType,
        /// Message creation timestamp
        #[serde(rename = "Created", with = "crate::timestamp")]
        created_at: DateTime<Utc>,
        /// Message text
        #[serde(rename = "Text", with = "string_empty_as_none")]
        text: Option<String>,
    },
    /// Recieved message
    Recieved {
        /// Username of the person who sent the message
        #[serde(rename = "From")]
        from: String,
        /// Type of the message
        #[serde(rename = "Media Type")]
        media_type: MediaType,
        /// Message creation timestamp
        #[serde(rename = "Created", with = "crate::timestamp")]
        created_at: DateTime<Utc>,
        /// Message text
        #[serde(rename = "Text", with = "string_empty_as_none")]
        text: Option<String>,
    }
}

impl Message {
    /// Getter for message recipient
    pub fn recipient(&self) -> String {
        match self {
            Message::Recieved { from, ..} => from.to_owned(),
            Message::Sent { to, .. } => to.to_owned(),
        }
    }
    /// Getter for message media type
    pub fn media_type(&self) -> MediaType {
        match self {
            Self::Recieved { media_type, .. } => media_type.clone(),
            Self::Sent { media_type, .. } => media_type.clone(),
        }
    }
    /// Getter for message timestamp
    pub fn created_at(&self) -> DateTime<Utc> {
        match self {
            Self::Recieved { created_at, .. } => created_at.clone(),
            Self::Sent { created_at, .. } => created_at.clone(),
        }
    }
    /// Getter for message text
    /// # Panics 
    /// Panics if the media type is not [`MediaType::Text`]
    pub fn text(&self) -> Option<String> {
        match self {
            Self::Recieved { text, .. } => text.clone(),
            Self::Sent { text, ..} => text.clone()
        }
    }
}

/// A type that contains all data parsed from a chat log file
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SnapchatData {
    /// Recieved chats that have been saved
    #[serde(rename = "Received Saved Chat History")]
    pub saved_recieved: Vec<Message>,
    /// Sent chats that have been saved
    #[serde(rename = "Sent Saved Chat History")]
    pub saved_sent: Vec<Message>,
}

impl SnapchatData {
    /// Parse a chat log file from a Reader
    /// # Examples 
    /// 
    /// ```no_run
    /// use std::fs::File;
    /// use std::io::Read;
    /// use snapchat_log_parser::types::SnapchatData;
    /// 
    /// let mut file = File::open("foo.txt").unwrap();
    /// let data = SnapchatData::from_reader(file).unwrap();
    /// ```
    pub fn from_reader<R: std::io::Read>(rdr: R) -> serde_json::Result<Self> {
        serde_json::from_reader(rdr)
    }
    /// Parse chat logs from a string
    /// # Examples
    /// 
    /// ```
    /// use snapchat_log_parser::types::SnapchatData;
    /// 
    /// // example data
    /// let json = r#"
    ///     {
    ///         "Received Saved Chat History": [{"From": "Felix422", "Media Type": "TEXT", "Created": "2021-01-01 12:00:00 UTC", "Text": "Test Message"}],
    ///         "Sent Saved Chat History": [{"To": "Felix422", "Media Type": "TEXT", "Created": "2021-01-01 12:00:00 UTC", "Text": "Test Message"}]
    ///     }
    ///  "#;
    /// let data = SnapchatData::from_str(json).unwrap();
    /// ```
    pub fn from_str<'a>(s: &'a str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
    /// Gets all messages from the chat with a specific user and sorts them
    pub fn get_user_chats(self, user: &str) -> Vec<Message> {
        let predicate = |x: &Message| x.recipient() == user;

        // filter both vectors to only include messages in a specific chat
        let rec = self.saved_recieved.into_iter().filter(&predicate);
        let sent = self.saved_sent.into_iter().filter(&predicate);

        // combine iterators
        let mut combined: Vec<Message> = rec.chain(sent).collect();
        // sort messages
        combined.sort_by(|a, b| a.created_at().partial_cmp(&b.created_at()).unwrap());
        combined
    }
    /// Gets a list of all users that are in the logs
    pub fn get_user_list(&self) -> HashSet<String> {
        let mut users = HashSet::new();
        for x in &self.saved_recieved {
            users.insert(x.recipient());
        }
        for x in &self.saved_sent {
            users.insert(x.recipient());
        }
        users
    }
}

/// Type of message
#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum MediaType {
    /// Normal text message
    #[serde(rename = "TEXT")]
    Text,
    /// Image
    #[serde(rename = "MEDIA")]
    Media,
    /// Sticker
    #[serde(rename = "STICKER")]
    Sticker,
    #[serde(rename = "SHARE")]
    Share,
    #[serde(rename = "NOTE")]
    Note,
    /// Shared Location
    #[serde(rename = "LOCATION")]
    Location,
}
