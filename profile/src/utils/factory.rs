use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

pub fn now() -> NaiveDateTime {
    Utc::now().naive_local()
}
pub fn id() -> Uuid {
    Uuid::new_v4()
}
