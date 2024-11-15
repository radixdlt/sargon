use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct PageResponse<T> {
    pub ledger_state: Option<LedgerState>,
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<T>,
}

impl<T> PageResponse<T> {
    pub fn new(
        ledger_state: impl Into<Option<LedgerState>>,
        total_count: impl Into<Option<u64>>,
        next_cursor: impl Into<Option<String>>,
        items: Vec<T>,
    ) -> Self {
        Self {
            ledger_state: ledger_state.into(),
            total_count: total_count.into(),
            next_cursor: next_cursor.into(),
            items,
        }
    }

    /// Create an empty PageResponse.
    /// Note: this is used to simulate a success response on some specific endpoints that are returning a 404 when
    /// requested for a virtual account. Ideally, the Gateway API should return a 200 with an empty array.
    /// Once it happens, this method should be removed.
    /// More info on thread: https://rdxworks.slack.com/archives/C06EBEA0SGY/p1731686360114749
    pub fn empty() -> Self {
        Self::new(None, None, None, vec![])
    }
}
