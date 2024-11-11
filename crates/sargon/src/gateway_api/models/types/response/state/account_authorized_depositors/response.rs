use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct AccountAuthorizedDepositorsResponse {
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<AccountAuthorizedDepositorsResponseItem>,
}
