use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreatingDevice {
    pub id: Uuid,
    pub date: NaiveDateTime,
    pub description: String,
}
