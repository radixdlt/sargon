use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct PageResponse<T> {
    pub ledger_state: LedgerState,
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<T>,
}

impl<T> PageResponse<T> {
    pub fn new(
        ledger_state: LedgerState,
        total_count: impl Into<Option<u64>>,
        next_cursor: impl Into<Option<String>>,
        items: Vec<T>,
    ) -> Self {
        Self {
            ledger_state,
            total_count: total_count.into(),
            next_cursor: next_cursor.into(),
            items,
        }
    }
}
