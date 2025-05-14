use std::time::SystemTime;
use chrono::{self, DateTime, Utc};

pub fn timestamp() -> String {
    let timestamp: DateTime<Utc> = SystemTime::now().into();
    format!("{}", timestamp.format("%+"))
}