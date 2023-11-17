use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

pub fn now() -> NaiveDateTime {
    Utc::now().naive_local()
}
pub fn id() -> Uuid {
    Uuid::new_v4()
}
fn date_to_string(dt: &NaiveDateTime, fmt: &str) -> String {
    dt.format(fmt).to_string()
}

pub fn iso8601(dt: &NaiveDateTime) -> String {
    date_to_string(dt, "%Y-%m-%d %H:%M:%S")
}

pub fn date(dt: &NaiveDateTime) -> String {
    date_to_string(dt, "%Y-%m-%d")
}
