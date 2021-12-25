use crate::types::*;
use lazy_static::lazy_static;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime, NaiveDateTime};

lazy_static!(
    static ref DATA: SnapchatData = SnapchatData {
        saved_recieved: vec![
            Message::Recieved {
                from: String::from("Felix422"),
                media_type: MediaType::Text,
                created_at: DateTime::<Utc>::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2021, 1, 1),
                        NaiveTime::from_hms(12, 0, 0),
                    ),
                    Utc
                ),
                text: Some(String::from("Test Message"))
            }
        ],
        saved_sent: vec![
            Message::Sent {
                to: String::from("Felix422"),
                media_type: MediaType::Text,
                created_at: DateTime::<Utc>::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2021, 1, 1),
                        NaiveTime::from_hms(12, 0, 0),
                    ),
                    Utc
                ),
                text: Some(String::from("Test Message"))
            }
        ]
    };
);

#[test]
fn parse_logs_from_str() {
    let json = r#"
        {
            "Received Saved Chat History": [{"From": "Felix422", "Media Type": "TEXT", "Created": "2021-01-01 12:00:00 UTC", "Text": "Test Message"}],
            "Sent Saved Chat History": [{"To": "Felix422", "Media Type": "TEXT", "Created": "2021-01-01 12:00:00 UTC", "Text": "Test Message"}]
        }
    "#;
    let parsed_data = SnapchatData::from_str(json).unwrap();
    assert_eq!(parsed_data, *DATA)
}

#[test]
fn getters() {
    assert_eq!(DATA.saved_recieved[0].recipient(), String::from("Felix422"));
    assert_eq!(DATA.saved_recieved[0].media_type(), MediaType::Text);
    assert_eq!(DATA.saved_recieved[0].created_at(), DateTime::<Utc>::from_utc(
        NaiveDateTime::new(
            NaiveDate::from_ymd(2021, 1, 1),
            NaiveTime::from_hms(12, 0, 0),
        ),
        Utc
    ));
    assert_eq!(DATA.saved_recieved[0].text(), Some(String::from("Test Message")));
}
