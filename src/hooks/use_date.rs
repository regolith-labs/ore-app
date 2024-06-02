use chrono::{Local, TimeZone};

pub fn use_datetime(timestamp: i64) -> String {
    // Format the DateTime<Utc> to a readable English string
    // %Y-%m-%d %H:%M:%S formats the date as "YYYY-MM-DD HH:MM:SS"
    let datetime = Local.timestamp_opt(timestamp, 0).unwrap();
    datetime.format("%b %d, %Y %H:%M:%S").to_string() // "Friday, October 01, 2021"
}
