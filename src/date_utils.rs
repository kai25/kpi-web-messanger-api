use chrono::{DateTime, Utc};

pub fn parse_date(date_str: &str) -> DateTime<Utc> {
    DateTime::from(DateTime::parse_from_rfc3339(date_str).unwrap())
}

pub fn date_to_str(date: DateTime<Utc>) -> String {
    date.to_rfc3339()
}