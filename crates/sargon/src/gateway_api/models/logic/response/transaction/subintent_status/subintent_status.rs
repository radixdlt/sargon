use crate::prelude::*;

impl SubintentStatusResponse {
    pub fn new(
        ledger_state: LedgerState,
        subintent_status: SubintentStatus,
        finalized_at_transaction_intent_hash: Option<String>,
    ) -> Self {
        Self {
            ledger_state,
            subintent_status,
            finalized_at_transaction_intent_hash,
        }
    }
}

#[cfg(test)]
impl SubintentStatusResponse {
    // Helper functions to create sample responses

    pub fn sample_unknown() -> Self {
        Self::new(
            LedgerState::sample_stokenet(),
            SubintentStatus::Unknown,
            None,
        )
    }

    pub fn sample_committed_success() -> Self {
        Self::committed_success(TransactionIntentHash::sample().to_string())
    }

    pub fn committed_success(
        finalized_at_transaction_intent_hash: impl Into<Option<String>>,
    ) -> Self {
        Self::new(
            LedgerState::sample_stokenet(),
            SubintentStatus::CommittedSuccess,
            finalized_at_transaction_intent_hash.into(),
        )
    }
}
