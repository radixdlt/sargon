use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct AccountResourcePreferencesResponse {
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<AccountResourcePreferencesResponseItem>,
}
