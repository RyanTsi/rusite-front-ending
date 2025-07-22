use chrono::{DateTime, Utc};
use chrono_tz::Asia::Shanghai;

pub fn format_date_cn(date: DateTime<Utc>) -> String {
    date.with_timezone(&Shanghai).format("%Y-%m-%d %H:%M").to_string()
}